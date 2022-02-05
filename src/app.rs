use pancurses::{endwin, initscr, noecho, set_title, resize_term, Input};

// Default values for window initialization
const DEFAULT_WIDTH: i32 = 100;
const DEFAULT_HEIGHT: i32 = 30;
const DEFAULT_TITLE: &str = "lunar App";

/// Main game App, everything is wrapped in here
pub struct App {
    width: i32,
    height: i32,
    title: String,
}

impl App {
    /// Default constructor to initialize App
    pub fn new() -> Self {
        return Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: String::from(DEFAULT_TITLE),
        };
    }

    /// Run current app
    pub fn run<I, U, R, E>(mut self, mut init: I, mut update: U, mut render: R, mut exit: E)
    where
        I: FnMut(&mut App) -> () + 'static,
        U: FnMut(&mut App) -> () + 'static,
        R: FnMut(&mut App) -> () + 'static,
        E: FnMut(&mut App) -> () + 'static,
    {
        let window = initscr();
        window.keypad(true);
        noecho();
        window.nodelay(true);

        init(&mut self);

        set_title(&self.title);
        resize_term(self.height, self.width);

        loop {
            match window.getch() {
                Some(Input::Character(c)) => {
                    if c == 'q' {
                        break;
                    }
                }
                Some(Input::KeyDC) => break,
                Some(_input) => {}
                None => (),
            }
            update(&mut self);

            window.clear();
            render(&mut self);
        }

        endwin();
        exit(&mut self);
    }
}
