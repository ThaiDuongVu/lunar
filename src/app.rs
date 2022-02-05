use pancurses::{curs_set, endwin, flash, initscr, noecho, resize_term, set_title, Input};

// Default values for window initialization
const DEFAULT_WIDTH: i32 = 100;
const DEFAULT_HEIGHT: i32 = 30;
const DEFAULT_TITLE: &str = "lunar App";

/// Main game App, everything is wrapped in here
pub struct App {
    width: i32,
    height: i32,
    title: String,
    do_quit: bool,
}

impl App {
    /// Default constructor to initialize App
    pub fn new() -> Self {
        return Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: String::from(DEFAULT_TITLE),
            do_quit: false,
        };
    }

    /// Invert App's color for a split second
    /// Warning: may cause seizure, please use with caution
    pub fn flash(&mut self) {
        flash();
    }

    /// Quit current App
    pub fn quit(&mut self) {
        self.do_quit = true;
    }

    /// Run current App
    pub fn run<I, U, R, E>(mut self, mut init: I, mut update: U, mut render: R, mut exit: E)
    where
        I: FnMut(&mut App) -> () + 'static,
        U: FnMut(&mut App) -> () + 'static,
        R: FnMut(&mut App) -> () + 'static,
        E: FnMut(&mut App) -> () + 'static,
    {
        let window = initscr();
        window.keypad(true);
        window.nodelay(true);
        noecho();

        curs_set(0); // TODO: Make this an option

        init(&mut self);

        set_title(&self.title);
        resize_term(self.height, self.width);

        loop {
            match window.getch() {
                Some(Input::Character(c)) => {
                    if c == 'q' {
                        self.quit();
                    }
                }
                Some(_input) => {}
                None => (),
            }
            if self.do_quit {
                break;
            }
            update(&mut self);

            window.clear();
            render(&mut self);
        }

        exit(&mut self);
        endwin();
    }
}
