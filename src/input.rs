use crate::types::mouse_button::MouseButton;

/// Input manager object
pub struct Input {
    pub is_key_down: Option<char>,
    pub is_mouse_down: Option<MouseButton>,
    pub on_mouse_clicked: Option<MouseButton>,
    pub on_mouse_release: Option<MouseButton>,
    pub on_mouse_double_clicked: Option<MouseButton>,
    pub on_mouse_triple_clicked: Option<MouseButton>,
}

impl Input {
    /// Default constructor to initialize Input manager
    pub fn new() -> Self {
        return Self {
            is_key_down: None,
            is_mouse_down: None,
            on_mouse_clicked: None,
            on_mouse_release: None,
            on_mouse_double_clicked: None,
            on_mouse_triple_clicked: None,
        };
    }

    pub fn is_character_down(&self, key: char) -> bool {
        return self.is_key_down != None && self.is_key_down.as_ref().unwrap() == &key;
    }

    pub fn is_character_up(&self, key: char) -> bool {
        return !self.is_character_down(key);
    }

    pub fn is_mouse_down(&self, mouse_button: MouseButton) -> bool {
        return self.is_mouse_down != None && self.is_mouse_down.as_ref().unwrap() == &mouse_button;
    }

    pub fn is_mouse_up(&self, mouse_button: MouseButton) -> bool {
        return !self.is_mouse_down(mouse_button);
    }

    pub fn on_mouse_clicked(&self, mouse_button: MouseButton) -> bool {
        return self.on_mouse_clicked != None
            && self.on_mouse_clicked.as_ref().unwrap() == &mouse_button;
    }
}
