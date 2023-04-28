/// Input manager object
pub struct Input {
    pub character_down: Option<char>,
}

impl Input {
    /// Default constructor to initialize Input manager
    pub fn new() -> Self {
        return Self {
            character_down: None,
        };
    }

    pub fn is_character_down(&self, character: char) -> bool {
        return self.character_down != None && self.character_down.as_ref().unwrap() == &character;
    }

    pub fn is_character_up(&self, character: char) -> bool {
        return !self.is_character_down(character);
    }
}
