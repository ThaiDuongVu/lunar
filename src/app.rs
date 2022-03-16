use crate::types::vector2int::Vector2Int;
use pancurses::{
    beep, curs_set, endwin, flash, initscr, noecho, resize_term, set_title, start_color,
};

#[derive(Clone, Copy)]
/// How to display the console cursor
pub enum CursorMode {
    Hidden = 0,
    Normal = 1,
    Block = 2,
}

// Default values for window initialization
const DEFAULT_WIDTH: i32 = 100;
const DEFAULT_HEIGHT: i32 = 30;
const DEFAULT_TITLE: &str = "lunar App";
const DEFAULT_CURSOR_MODE: CursorMode = CursorMode::Hidden;

/// Main game App, everything is wrapped in here
pub struct App {
    width: i32,
    height: i32,
    title: String,
    cursor_mode: CursorMode,
    background: Option<char>,
    do_quit: bool,
    window: Option<pancurses::Window>,
}

impl App {
    /// Default constructor to initialize App
    pub fn new() -> Self {
        return Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: String::from(DEFAULT_TITLE),
            cursor_mode: DEFAULT_CURSOR_MODE,
            background: None,
            do_quit: false,
            window: None,
        };
    }

    /// Set App console window width
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
        resize_term(self.height, self.width);
    }

    /// Get App console window width
    pub fn width(&self) -> i32 {
        return self.width;
    }

    /// Set App console window height
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
        resize_term(self.height, self.width);
    }

    /// Get App console height
    pub fn height(&self) -> i32 {
        return self.height;
    }

    /// Set App console window width and height
    pub fn set_size(&mut self, size: Vector2Int) {
        self.width = size.x;
        self.height = size.y;
        resize_term(self.height, self.width);
    }

    /// Return current App's console window size as a Vector2Int
    pub fn size(&self) -> Vector2Int {
        return Vector2Int {
            x: self.width,
            y: self.height,
        };
    }

    /// Set App console window title
    pub fn set_title(&mut self, title: String) {
        set_title(&title);
    }

    /// Get App console window title
    pub fn title(&self) -> &str {
        return &self.title;
    }

    /// Set current App's cursor display mode
    pub fn set_cursor(&mut self, mode: CursorMode) {
        self.cursor_mode = mode;
        curs_set(self.cursor_mode as i32);
    }

    /// Get current App's cursor display mode
    pub fn cursor_mode(&self) -> CursorMode {
        return self.cursor_mode;
    }

    /// Set current App's background character
    pub fn set_background(&mut self, background_char: char) {
        self.background = Some(background_char);
        self.window.as_ref().unwrap().bkgdset(background_char);
        self.window.as_ref().unwrap().clear();
    }

    /// Set current App's background to empty
    pub fn clear_background(&mut self) {
        self.background = None;
        self.window.as_ref().unwrap().bkgdset(0 as u64);
        self.window.as_ref().unwrap().clear();
    }

    /// Get current App's background character if there is a background set
    pub fn get_background(&self) -> Option<char> {
        return self.background;
    }

    /// Invert App's color for a split second
    /// Warning: may cause seizure, please use with caution
    pub fn flash(&self) {
        flash();
    }

    /// Play native OS's beep sound
    /// Warning: can be very annoying, please use with caution
    pub fn beep(&self) {
        beep();
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
        self.window = Some(initscr());
        self.window.as_ref().unwrap().keypad(true);
        self.window.as_ref().unwrap().nodelay(true);
        noecho();
        start_color();

        curs_set(self.cursor_mode as i32); // Set default cursor mode
        set_title(&self.title); // Set default window title
        resize_term(self.height, self.width); // Set default window size

        init(&mut self);

        loop {
            match self.window.as_ref().unwrap().getch() {
                Some(pancurses::Input::Character(character)) => {
                    if character == 'q' {
                        self.quit();
                    } else if character == 'b' {
                        self.set_background('b');
                    } else if character == 'c' {
                        self.clear_background();
                    }
                }
                Some(pancurses::Input::KeyEnter) => {
                    println!("interesting");
                }
                Some(_input) => {}
                None => (),
            }

            if self.do_quit {
                break;
            }
            update(&mut self);

            // Update renders
            self.window.as_ref().unwrap().refresh();
            // window.border(test, 'b', 'c', 'd', 'e', 'f', 'g', 'h'); // TODO: Extract this to a user function
            render(&mut self);
        }

        exit(&mut self);
        endwin();
    }
}
