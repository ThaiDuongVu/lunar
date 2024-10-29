use super::game_object::GameObject;
use crate::app::App;
use crate::types::vector2int::Vector2Int;
use ndarray::Array2;

#[derive(Copy, Clone)]
/// A 2D character-based object
pub struct CharObject {
    game_object: GameObject,
    is_visible: bool,
    char_map: *const Array2<char>,
}

impl CharObject {
    /// Default constructor
    pub fn new(char_map: *const Array2<char>) -> Self {
        return Self {
            game_object: GameObject::new(),
            is_visible: true,
            char_map,
        };
    }

    /// Return object static state
    pub fn get_static(&self) -> bool {
        return self.game_object.get_static();
    }

    /// Set object static
    pub fn set_static(&mut self, value: bool) -> CharObject {
        self.game_object.set_static(value);
        return *self;
    }

    /// Get object position
    pub fn get_position(&self) -> Vector2Int {
        return self.game_object.get_position();
    }

    /// Move object to a new position
    pub fn move_to(&mut self, new_position: Vector2Int) -> CharObject {
        self.game_object.move_to(new_position);
        return *self;
    }

    /// Move object by a vector
    pub fn move_by(&mut self, delta: Vector2Int) -> CharObject {
        self.game_object.move_by(delta);
        return *self;
    }

    /// Get parent GameObject
    pub fn get_game_object(&self) -> GameObject {
        return self.game_object;
    }

    /// Return object visibility
    pub fn get_visible(&self) -> bool {
        return self.is_visible;
    }

    /// Set object visibility
    pub fn set_visible(&mut self, value: bool) -> CharObject {
        self.is_visible = value;
        return *self;
    }

    /// Get char map width
    pub fn get_width(&self) -> usize {
        let map = unsafe { self.char_map.as_ref().unwrap() };
        return map.shape()[1];
    }

    /// Get char map height
    pub fn get_height(&self) -> usize {
        let map = unsafe { self.char_map.as_ref().unwrap() };
        return map.shape()[0];
    }

    /// Render object on App window
    pub fn render(&self, app: &mut App) {
        if !self.is_visible {
            return;
        }

        // Render chars in char map
        let map = unsafe { self.char_map.as_ref().unwrap() };
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                app.window.mvaddch(
                    self.get_position().y + y as i32,
                    self.get_position().x + x as i32,
                    map[[y, x]],
                );
            }
        }
    }
}
