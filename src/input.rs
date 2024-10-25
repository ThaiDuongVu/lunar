use pancurses::getmouse;

// Input manager
pub struct Input {
    pub key_down: char,
}

impl Input {
    /// Default constructor
    pub fn new() -> Self {
        return Self {
            key_down: ' ',
        };
    }

    // Query for input from App
    pub fn query(&mut self, getch: Option<pancurses::Input>) {
        match getch {
            Some(pancurses::Input::KeyMouse) => {
                if let Ok(_mouse_event) = getmouse() {
                    // TODO: Use mouse_event.bstate to handle mouse input
                }
            }
            Some(pancurses::Input::Character(character)) => {
                // Handle keyboard input down
                self.key_down = character;
            }
            Some(_input) => {}
            None => {
                // Handle keyboard input up
                self.key_down = ' ';
            }
        }
    }
}
