use ndarray::Array2;

use crate::{app::App, types::vector2int::Vector2Int};

#[derive(Copy, Clone)]
/// An object that exist in the game world
pub struct GameObject {
    is_visible: bool,
    is_static: bool,

    position: Vector2Int,
    map: *const Array2<char>,
}

impl GameObject {
    /// Default constructor
    pub fn new(map: *const Array2<char>) -> Self {
        return Self {
            is_visible: true,
            is_static: false,

            position: Vector2Int::zero(),
            map,
        };
    }

    /// Return object visibility
    pub fn get_visible(&self) -> bool {
        return self.is_visible;
    }

    /// Set object visibility
    pub fn set_visible(&mut self, value: bool) -> GameObject {
        self.is_visible = value;
        return *self;
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

    /// Render object on App window
    pub fn render(&self, app: &mut App) {
        if !self.is_visible {
            return;
        }
        let map = unsafe { self.map.as_ref().unwrap() };
        for y in 0..map.shape()[0] {
            for x in 0..map.shape()[1] {
                app.window.mvaddch(
                    self.position.y + y as i32,
                    self.position.x + x as i32,
                    map[[y, x]],
                );
            }
        }
    }
}
