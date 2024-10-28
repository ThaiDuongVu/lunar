use std::{thread::sleep, time::Duration};

use crate::{input::Input, types::vector2int::Vector2Int};
use pancurses::{
    beep, curs_set, endwin, flash, initscr, mousemask, noecho, resize_term, set_title, start_color,
    Window, ALL_MOUSE_EVENTS, REPORT_MOUSE_POSITION,
};

#[derive(Clone, Copy)]
/// How to display the console cursor
pub enum CursorMode {
    Hidden = 0,
    Normal = 1,
    Block = 2,
}

// Default values for window initialization
pub const DEFAULT_WIDTH: i32 = 100;
pub const DEFAULT_HEIGHT: i32 = 30;
pub const DEFAULT_TITLE: &str = "lunar App";
pub const DEFAULT_CURSOR_MODE: CursorMode = CursorMode::Hidden;
pub const DEFAULT_BACKGROUND: u64 = 0;
pub const DEFAULT_BORDER: u64 = 0;
pub const DEFAULT_CORNER: u64 = 0;
pub const DEFAULT_FRAMERATE: u32 = 60;

/// Main program App, everything is wrapped in here
pub struct App {
    // Window properties
    width: i32,
    height: i32,
    title: String,
    background: u64,
    cursor_mode: CursorMode,
    frame_time: f32,

    // Borders
    border_left: u64,
    border_right: u64,
    border_top: u64,
    border_bottom: u64,

    // Corners
    corner_top_left: u64,
    corner_top_right: u64,
    corner_bottom_left: u64,
    corner_bottom_right: u64,

    do_quit: bool,
    pub window: Window,
    pub input: Input,
}

impl App {
    /// Default constructor to initialize App
    pub fn new() -> Self {
        return Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: String::from(DEFAULT_TITLE),
            background: DEFAULT_BACKGROUND,
            cursor_mode: DEFAULT_CURSOR_MODE,
            frame_time: 1 as f32 / DEFAULT_FRAMERATE as f32,

            border_left: DEFAULT_BORDER,
            border_right: DEFAULT_BORDER,
            border_top: DEFAULT_BORDER,
            border_bottom: DEFAULT_BORDER,

            corner_top_left: DEFAULT_CORNER,
            corner_top_right: DEFAULT_CORNER,
            corner_bottom_left: DEFAULT_CORNER,
            corner_bottom_right: DEFAULT_CORNER,

            do_quit: false,
            window: initscr(),
            input: Input::new(),
        };
    }

    //#region Constant getters
    pub fn get_default_width() -> i32 {
        return DEFAULT_WIDTH;
    }

    pub fn get_default_height() -> i32 {
        return DEFAULT_HEIGHT;
    }

    pub fn get_default_title() -> String {
        return DEFAULT_TITLE.to_owned();
    }

    pub fn get_default_cursor_mode() -> CursorMode {
        return DEFAULT_CURSOR_MODE;
    }

    pub fn get_default_framerate() -> u32 {
        return DEFAULT_FRAMERATE;
    }
    //#endregion

    /// Set App console window width
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
        resize_term(self.height, self.width);
    }

    /// Get App console window width
    pub fn get_width(&self) -> i32 {
        return self.width;
    }

    /// Set App console window height
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
        resize_term(self.height, self.width);
    }

    /// Get App console height
    pub fn get_height(&self) -> i32 {
        return self.height;
    }

    /// Set App console window width and height
    pub fn set_size(&mut self, size: Vector2Int) {
        self.width = size.x;
        self.height = size.y;
        resize_term(self.height, self.width);
    }

    /// Return current App's console window size as a Vector2Int
    pub fn get_size(&self) -> Vector2Int {
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
    pub fn get_title(&self) -> &str {
        return &self.title;
    }

    /// Set current App's cursor display mode
    pub fn set_cursor_mode(&mut self, mode: CursorMode) {
        self.cursor_mode = mode;
        curs_set(self.cursor_mode as i32);
    }

    /// Get current App's cursor display mode
    pub fn get_cursor_mode(&self) -> CursorMode {
        return self.cursor_mode;
    }

    /// Set current App's background character
    pub fn set_background(&mut self, background_char: char) {
        self.background = background_char as u64;
        self.window.bkgdset(self.background);
        self.window.clear();
    }

    /// Set current App's background to empty
    pub fn clear_background(&mut self) {
        self.background = DEFAULT_BACKGROUND;
        self.window.bkgdset(self.background);
        self.window.clear();
    }

    /// Get current App's background character if there is a background set
    pub fn get_background(&self) -> char {
        return self.background as u8 as char;
    }

    //#region
    /// Update all App's borders and corners based on current border values
    fn update_borders_corners(&mut self) {
        self.window.border(
            self.border_left,
            self.border_right,
            self.border_top,
            self.border_bottom,
            self.corner_top_left,
            self.corner_top_right,
            self.corner_bottom_left,
            self.corner_bottom_right,
        );
    }

    /// Set current App's left side border
    pub fn set_border_left(&mut self, border_char: char) {
        self.border_left = border_char as u64;
        self.update_borders_corners();
    }

    /// Set current App's right side border
    pub fn set_border_right(&mut self, border_char: char) {
        self.border_right = border_char as u64;
        self.update_borders_corners();
    }

    /// Set current App's top side border
    pub fn set_border_top(&mut self, border_char: char) {
        self.border_top = border_char as u64;
        self.update_borders_corners();
    }

    /// Set current App's bottom side border
    pub fn set_border_bottom(&mut self, border_char: char) {
        self.border_bottom = border_char as u64;
        self.update_borders_corners();
    }

    /// Set all current App's borders
    pub fn set_all_borders(&mut self, border_char: char) {
        self.border_top = border_char as u64;
        self.border_bottom = border_char as u64;
        self.border_left = border_char as u64;
        self.border_right = border_char as u64;
        self.update_borders_corners();
    }

    /// Clear current App's left border
    pub fn clear_border_left(&mut self) {
        self.border_left = DEFAULT_BORDER;
        self.update_borders_corners();
    }

    /// Clear current App's right border
    pub fn clear_border_right(&mut self) {
        self.border_right = DEFAULT_BORDER;
        self.update_borders_corners();
    }

    /// Clear current App's top border
    pub fn clear_border_top(&mut self) {
        self.border_top = DEFAULT_BORDER;
        self.update_borders_corners();
    }

    /// Clear current App's bottom border
    pub fn clear_border_bottom(&mut self) {
        self.border_bottom = DEFAULT_BORDER;
        self.update_borders_corners();
    }

    /// Clear all App borders
    pub fn clear_all_borders(&mut self) {
        self.border_left = DEFAULT_BORDER;
        self.border_right = DEFAULT_BORDER;
        self.border_top = DEFAULT_BORDER;
        self.border_bottom = DEFAULT_BORDER;
        self.update_borders_corners();
    }

    /// Set current App's top left corner
    pub fn set_corner_top_left(&mut self, corner_char: char) {
        self.corner_top_left = corner_char as u64;
        self.update_borders_corners();
    }

    /// Set current App's top right corner
    pub fn set_corner_top_right(&mut self, corner_char: char) {
        self.corner_top_right = corner_char as u64;
        self.update_borders_corners();
    }

    /// Set current App's bottom left corner
    pub fn set_corner_bottom_left(&mut self, corner_char: char) {
        self.corner_bottom_left = corner_char as u64;
        self.update_borders_corners();
    }

    /// Set current App's bottom right corner
    pub fn set_corner_bottom_right(&mut self, corner_char: char) {
        self.corner_bottom_right = corner_char as u64;
        self.update_borders_corners();
    }

    /// Set all current App's corners
    pub fn set_all_corners(&mut self, corner_char: char) {
        self.corner_top_left = corner_char as u64;
        self.corner_top_right = corner_char as u64;
        self.corner_bottom_left = corner_char as u64;
        self.corner_bottom_right = corner_char as u64;
        self.update_borders_corners();
    }

    /// Clear current App's top left corner
    pub fn clear_corner_top_left(&mut self) {
        self.corner_top_left = DEFAULT_CORNER;
        self.update_borders_corners();
    }

    /// Clear current App's top right corner
    pub fn clear_corner_top_right(&mut self) {
        self.corner_top_right = DEFAULT_CORNER;
        self.update_borders_corners();
    }

    /// Clear current App's bottom left corner
    pub fn clear_corner_bottom_left(&mut self) {
        self.corner_bottom_left = DEFAULT_CORNER;
        self.update_borders_corners();
    }

    /// Clear current App's bottom right corner
    pub fn clear_corner_bottom_right(&mut self) {
        self.corner_bottom_right = DEFAULT_CORNER;
        self.update_borders_corners();
    }

    /// Clear all App corners
    pub fn clear_all_corners(&mut self) {
        self.corner_top_left = DEFAULT_CORNER;
        self.corner_top_right = DEFAULT_CORNER;
        self.corner_bottom_left = DEFAULT_CORNER;
        self.corner_bottom_right = DEFAULT_CORNER;
        self.update_borders_corners();
    }
    //#endregion

    /// Set App's framerate
    pub fn set_framerate(&mut self, framerate: u32) {
        self.frame_time = 1 as f32 / framerate as f32;
    }

    /// Invert App's color for a split second
    ///
    /// Warning: may cause seizure, please use with caution
    pub fn flash(&self) {
        flash();
    }

    /// Play native OS's beep sound
    ///
    /// Warning: can be annoying, please use with caution
    pub fn beep(&self) {
        beep();
    }

    /// Clear the entire screen
    pub fn clear(&self) {
        self.window.clear();
    }

    /// Quit current App
    pub fn quit(&mut self) {
        self.do_quit = true;
    }

    /// Run current App
    pub fn run<I, U, R, E>(mut self, mut init: I, mut update: U, mut render: R, mut exit: E)
    where
        I: FnMut(&mut App) -> (),
        U: FnMut(&mut App) -> (),
        R: FnMut(&mut App) -> (),
        E: FnMut(&mut App) -> (),
    {
        // Initialize current window and set default values
        self.window.keypad(true);
        self.window.nodelay(true);
        // Listen to all mouse events
        mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, None);
        noecho();
        start_color();

        // Set cursor mode
        self.set_cursor_mode(DEFAULT_CURSOR_MODE);
        // Set title
        self.set_title(DEFAULT_TITLE.to_string());
        // Set terminal size
        self.set_size(Vector2Int {
            x: DEFAULT_WIDTH,
            y: DEFAULT_HEIGHT,
        });
        // Set background, borders & corners
        self.clear_background();
        self.clear_all_borders();
        self.clear_all_corners();

        // User-defined initialization
        init(&mut self);

        loop {
            // Query for input from App
            self.input.query(self.window.getch());

            // User-defined update
            update(&mut self);

            // User-defined render then refresh the screen
            self.window.clear();
            self.update_borders_corners();
            render(&mut self);

            if self.do_quit {
                break;
            }

            // Handle frame rate
            sleep(Duration::from_secs_f32(self.frame_time));
        }

        // User-defined exit
        exit(&mut self);
        endwin();
    }
}
