use crate::types::vector2int::Vector2Int;

#[derive(Copy, Clone)]
/// An object that exist in the game world
pub struct GameObject {
    is_static: bool,
    position: Vector2Int,
}

impl GameObject {
    /// Default constructor
    pub fn new() -> Self {
        return Self {
            is_static: false,
            position: Vector2Int::zero(),
        };
    }

    /// Return object static state
    pub fn get_static(&self) -> bool {
        return self.is_static;
    }

    /// Set object static
    pub fn set_static(&mut self, value: bool) -> GameObject {
        self.is_static = value;
        return *self;
    }

    /// Get object position
    pub fn get_position(&self) -> Vector2Int {
        return self.position;
    }

    /// Move object to a new position
    pub fn move_to(&mut self, new_position: Vector2Int) -> GameObject {
        if self.is_static {
            return *self;
        }
        self.position = new_position;
        return *self;
    }

    /// Move object by a vector
    pub fn move_by(&mut self, delta: Vector2Int) -> GameObject {
        if self.is_static {
            return *self;
        }
        self.position.translate(delta);
        return *self;
    }
}
