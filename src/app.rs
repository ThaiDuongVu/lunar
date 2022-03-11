use crate::types::vector2int::Vector2Int;
use pancurses::{
    beep, curs_set, endwin, flash, initscr, noecho, resize_term, set_title, start_color, Input,
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

    do_quit: bool,
}

impl App {
    /// Default constructor to initialize App
    pub fn new() -> Self {
        return Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: String::from(DEFAULT_TITLE),

            cursor_mode: DEFAULT_CURSOR_MODE,

            do_quit: false,
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
        let window = initscr();
        window.keypad(true);
        window.nodelay(true);
        noecho();
        start_color();

        curs_set(self.cursor_mode as i32); // Set default cursor mode
        set_title(&self.title); // Set default window title
        resize_term(self.height, self.width); // Set default window size

        init(&mut self);

        loop {
            match window.getch() {
                Some(Input::Character(character)) => {
                    if character == 'q' {
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
