use pancurses::getmouse;

#[derive(PartialEq, Clone, Copy)]
pub enum SpecialKey {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
pub enum MouseEvent {
    None = 0,
    LeftMouseUp = 1,
    LeftMouseDown = 2,
    LeftMouseClick = 4,
    LeftMouseDoubleClick = 8,
    MiddleMouseUp = 32,
    MiddleMouseDown = 64,
    MiddleMouseClick = 128,
    MiddleMouseDoubleClick = 256,
    RightMouseUp = 1024,
    RightMouseDown = 2048,
    RightMouseClick = 4096,
    RightMouseDoubleClick = 8192,
}

// Input manager
pub struct Input {
    char_key_down: char,
    special_key_down: SpecialKey,
    mouse_event: MouseEvent,
}

impl Input {
    /// Default constructor
    pub fn new() -> Self {
        return Self {
            char_key_down: ' ',
            special_key_down: SpecialKey::None,
            mouse_event: MouseEvent::None,
        };
    }

    // Query for input from App
    pub fn query(&mut self, getch: Option<pancurses::Input>) {
        match getch {
            // Handle mouse input
            Some(pancurses::Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    match mouse_event.bstate {
                        1 => self.mouse_event = MouseEvent::LeftMouseUp,
                        2 => self.mouse_event = MouseEvent::LeftMouseDown,
                        4 => self.mouse_event = MouseEvent::LeftMouseClick,
                        8 => self.mouse_event = MouseEvent::LeftMouseDoubleClick,
                        32 => self.mouse_event = MouseEvent::MiddleMouseUp,
                        64 => self.mouse_event = MouseEvent::MiddleMouseDown,
                        128 => self.mouse_event = MouseEvent::MiddleMouseClick,
                        256 => self.mouse_event = MouseEvent::MiddleMouseDoubleClick,
                        1024 => self.mouse_event = MouseEvent::RightMouseUp,
                        2048 => self.mouse_event = MouseEvent::RightMouseDown,
                        4096 => self.mouse_event = MouseEvent::RightMouseClick,
                        8192 => self.mouse_event = MouseEvent::RightMouseDoubleClick,
                        _ => self.mouse_event = MouseEvent::None,
                    }
                }
            }
            // Handle character key input
            Some(pancurses::Input::Character(char)) => {
                self.char_key_down = char;
            }
            // Handle special key input
            Some(pancurses::Input::KeyUp) => {
                self.special_key_down = SpecialKey::Up;
            }
            Some(pancurses::Input::KeyDown) => {
                self.special_key_down = SpecialKey::Down;
            }
            Some(pancurses::Input::KeyLeft) => {
                self.special_key_down = SpecialKey::Left;
            }
            Some(pancurses::Input::KeyRight) => {
                self.special_key_down = SpecialKey::Right;
            }
            // Handle input up
            Some(_key) => {
                self.char_key_down = ' ';
                self.special_key_down = SpecialKey::None;
            }
            None => {
                self.char_key_down = ' ';
                self.special_key_down = SpecialKey::None;
            }
        }
    }

    pub fn get_char_key_down(&self) -> char {
        return self.char_key_down;
    }

    pub fn get_special_key_dowb(&self) -> SpecialKey {
        return self.special_key_down;
    }

    pub fn get_mouse_event(&self) -> MouseEvent {
        return self.mouse_event;
    }

    pub fn is_char_key_down(&self, key: char) -> bool {
        return key == self.char_key_down;
    }

    pub fn is_char_key_up(&self, key: char) -> bool {
        return key != self.char_key_down;
    }

    pub fn is_special_key_down(&self, key: SpecialKey) -> bool {
        return key == self.special_key_down;
    }

    pub fn is_special_key_up(&self, key: SpecialKey) -> bool {
        return key != self.special_key_down;
    }
}
