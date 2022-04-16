/// Input manager object
pub struct Input {
    pub current_key_down: Option<char>,
}

/// Input keys
pub enum Keys {
    Q,
}

impl Input {
    /// Default constructor to initialize Input manager
    pub fn new() -> Self {
        return Self {
            current_key_down: None,
        };
    }

    pub fn is_key_down(&self, key: char) -> bool {
        self.current_key_down != None && self.current_key_down.as_ref().unwrap() == &key
    }

    pub fn is_key_up(&self, key: char) -> bool {
        !self.is_key_down(key)
    }
}
