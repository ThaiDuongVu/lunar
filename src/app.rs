use crate::input::Input;
use crate::types::mouse_button::MouseButton;
use crate::types::vector2int::Vector2Int;
use pancurses::{
    beep, curs_set, endwin, flash, getmouse, initscr, mousemask, noecho, resize_term, set_title,
    start_color, Window, ALL_MOUSE_EVENTS, REPORT_MOUSE_POSITION,
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
const DEFAULT_BACKGROUND: u64 = 0;
const DEFAULT_BORDER: u64 = 128;
const DEFAULT_CORNER: u64 = 128;

/// Main game App, everything is wrapped in here
pub struct App {
    width: i32,
    height: i32,
    title: String,
    cursor_mode: CursorMode,
    do_quit: bool,
    pub window: Option<Window>,
    pub input: Input,

    background: u64,

    border_left: u64,
    border_right: u64,
    border_top: u64,
    border_bottom: u64,

    corner_top_left: u64,
    corner_top_right: u64,
    corner_bottom_left: u64,
    corner_bottom_right: u64,
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
            window: None,
            input: Input::new(),

            background: DEFAULT_BACKGROUND,

            border_left: DEFAULT_BORDER,
            border_right: DEFAULT_BORDER,
            border_top: DEFAULT_BORDER,
            border_bottom: DEFAULT_BORDER,

            corner_top_left: DEFAULT_CORNER,
            corner_top_right: DEFAULT_CORNER,
            corner_bottom_left: DEFAULT_CORNER,
            corner_bottom_right: DEFAULT_CORNER,
        };
    }

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
        self.window.as_ref().unwrap().bkgdset(self.background);
        self.window.as_ref().unwrap().clear();
    }

    /// Set current App's background to empty
    pub fn clear_background(&mut self) {
        self.background = DEFAULT_BACKGROUND;
        self.window.as_ref().unwrap().bkgdset(self.background);
        self.window.as_ref().unwrap().clear();
    }

    /// Get current App's background character if there is a background set
    pub fn get_background(&self) -> char {
        return self.background as u8 as char;
    }

    /// Update all App's borders and corners based on current border values
    fn update_borders_corners(&mut self) {
        self.window.as_ref().unwrap().border(
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

    /// Set all App borders AND corners
    pub fn set_all_borders_corners(&mut self, border_corner_char: char) {
        self.set_all_borders(border_corner_char);
        self.set_all_corners(border_corner_char);
    }

    /// Invert App's color for a split second
    ///
    /// Warning: may cause seizure, please use with caution
    pub fn flash(&self) {
        flash();
    }

    /// Play native OS's beep sound
    ///
    /// Warning: can be very annoying, please use with caution
    pub fn beep(&self) {
        beep();
    }

    /// Clear the entire screen
    pub fn erase(&self) {
        self.window.as_ref().unwrap().erase();
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
        // Initialize current window and set default values
        self.window = Some(initscr());
        self.window.as_ref().unwrap().keypad(true);
        self.window.as_ref().unwrap().nodelay(true);
        // Listen to all mouse events
        mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, None);
        noecho();
        start_color();

        curs_set(self.cursor_mode as i32);
        set_title(&self.title);
        resize_term(self.height, self.width);
        self.clear_background();

        // User-defined initialization
        init(&mut self);

        loop {
            match self.window.as_ref().unwrap().getch() {
                Some(pancurses::Input::KeyMouse) => {
                    if let Ok(mouse_event) = getmouse() {
                        self.input.is_mouse_down = Some(MouseButton::from(mouse_event.bstate));
                        self.input.on_mouse_clicked = Some(MouseButton::from(mouse_event.bstate));
                    };
                }
                Some(pancurses::Input::Character(character)) => {
                    self.input.is_key_down = Some(character);
                }
                Some(_input) => {}
                None => {
                    self.input.is_key_down = None;
                    self.input.on_mouse_clicked = None;
                }
            }

            if self.do_quit {
                break;
            }

            // User-defined update
            update(&mut self);

            // Refresh screen and call user-defined render
            self.window.as_ref().unwrap().refresh();
            render(&mut self);
        }

        // User-defined exit
        exit(&mut self);
        endwin();
    }
}
