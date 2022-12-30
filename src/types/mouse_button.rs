use pancurses;

#[derive(PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    None,
}

impl From<u32> for MouseButton {
    fn from(origin: u32) -> Self {
        match origin {
            pancurses::BUTTON1_PRESSED => return MouseButton::Left,
            pancurses::BUTTON1_CLICKED => return MouseButton::Left,
            pancurses::BUTTON1_RELEASED => return MouseButton::Left,
            pancurses::BUTTON1_DOUBLE_CLICKED => return MouseButton::Left,
            pancurses::BUTTON1_TRIPLE_CLICKED => return MouseButton::Left,

            pancurses::BUTTON2_PRESSED => return MouseButton::Middle,
            pancurses::BUTTON2_CLICKED => return MouseButton::Middle,
            pancurses::BUTTON2_RELEASED => return MouseButton::Middle,
            pancurses::BUTTON2_DOUBLE_CLICKED => return MouseButton::Middle,
            pancurses::BUTTON2_TRIPLE_CLICKED => return MouseButton::Middle,

            pancurses::BUTTON3_PRESSED => return MouseButton::Right,
            pancurses::BUTTON3_CLICKED => return MouseButton::Right,
            pancurses::BUTTON3_RELEASED => return MouseButton::Right,
            pancurses::BUTTON3_DOUBLE_CLICKED => return MouseButton::Right,
            pancurses::BUTTON3_TRIPLE_CLICKED => return MouseButton::Right,

            _ => return MouseButton::None,
        };
    }
}

impl std::fmt::Display for MouseButton {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MouseButton::Left => write!(f, "Left"),
            MouseButton::Right => write!(f, "Right"),
            MouseButton::Middle => write!(f, "Middle"),
            MouseButton::None => write!(f, "None"),
        }
    }
}
