use pancurses::getmouse;

#[derive(PartialEq, Clone, Copy)]
pub enum SpecialKey {
    None,
    Up,
    Down,
    Left,
    Right,
}

// Input manager
pub struct Input {
    pub char_key_down: char,
    pub special_key_down: SpecialKey,
}

impl Input {
    /// Default constructor
    pub fn new() -> Self {
        return Self {
            char_key_down: ' ',
            special_key_down: SpecialKey::None,
        };
    }

    // Query for input from App
    pub fn query(&mut self, getch: Option<pancurses::Input>) {
        match getch {
            // Handle mouse input
            Some(pancurses::Input::KeyMouse) => {
                if let Ok(_mouse_event) = getmouse() {
                    // TODO: Use mouse_event.bstate to handle mouse input
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

    pub fn is_char_key_down(&self, key: char) -> bool {
        return key == self.char_key_down;
    }

    pub fn is_special_key_down(&self, key: SpecialKey) -> bool {
        return key == self.special_key_down;
    }
}
