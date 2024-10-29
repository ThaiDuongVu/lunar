use super::game_object::GameObject;
use crate::app::App;
use crate::types::vector2int::Vector2Int;

#[derive(Copy, Clone)]
/// A 1D string-based object
pub struct TextObject {
    game_object: GameObject,
    is_visible: bool,
    text: *const String,
}

impl TextObject {
    /// Default constructor
    pub fn new(text: *const String) -> Self {
        return Self {
            game_object: GameObject::new(),
            is_visible: true,
            text
        };
    }

    /// Return object static state
    pub fn get_static(&self) -> bool {
        return self.game_object.get_static();
    }

    /// Set object static
    pub fn set_static(&mut self, value: bool) -> TextObject {
        self.game_object.set_static(value);
        return *self;
    }

    /// Get object position
    pub fn get_position(&self) -> Vector2Int {
        return self.game_object.get_position();
    }

    /// Move object to a new position
    pub fn move_to(&mut self, new_position: Vector2Int) -> TextObject {
        self.game_object.move_to(new_position);
        return *self;
    }

    /// Move object by a vector
    pub fn move_by(&mut self, delta: Vector2Int) -> TextObject {
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
    pub fn set_visible(&mut self, value: bool) -> TextObject {
        self.is_visible = value;
        return *self;
    }

    /// Render object on App window
    pub fn render(&self, app: &mut App) {
        if !self.is_visible {
            return;
        }

        // Render string text
        let text = unsafe { self.text.as_ref().unwrap() };
        app.window.mvaddstr(self.get_position().y, self.get_position().x, text);
    }
}
