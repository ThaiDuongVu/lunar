use lunar::{
    app::App, entities::game_object::GameObject, input::SpecialKey, types::vector2int::Vector2Int,
};
use ndarray::array;
use std::cell::Cell;

fn main() {
    // Create a lunar app
    let app = App::new();

    let divider_map = array![
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
        ['|'],
    ];
    let divider = Cell::new(GameObject::new(&divider_map));

    let p1_map = array![['['], ['['], ['['], ['['], ['[']];
    let player1 = Cell::new(GameObject::new(&p1_map));

    let p2_map = array![[']'], [']'], [']'], [']'], [']']];
    let player2 = Cell::new(GameObject::new(&p2_map));

    // Initialize App
    let init = |app: &mut App| {
        app.set_width(101);
        app.set_height(29);
        app.set_title("Pong".to_string());

        divider.set(divider.get().move_to(Vector2Int { x: 50, y: 0 }));
        player1.set(player1.get().move_to(Vector2Int { x: 3, y: 12 }));
        player2.set(player2.get().move_to(Vector2Int { x: 97, y: 12 }));
    };

    // Update App
    let update = |app: &mut App| {
        if app.input.is_char_key_down('q') {
            app.quit();
        }

        // Move player 1 using WS keys
        if app.input.is_char_key_down('w') {
            player1.set(player1.get().move_by(Vector2Int::down()));
        } else if app.input.is_char_key_down('s') {
            player1.set(player1.get().move_by(Vector2Int::up()));
        }

        // Move player 2 using arrow keys
        if app.input.is_special_key_down(SpecialKey::Up) {
            player2.set(player2.get().move_by(Vector2Int::down()));
        } else if app.input.is_special_key_down(SpecialKey::Down) {
            player2.set(player2.get().move_by(Vector2Int::up()));
        }
    };

    // Render objects on App window
    let render = |app: &mut App| {
        divider.get().render(app);
        player1.get().render(app);
        player2.get().render(app);
    };

    // Exit App
    let exit = |_app: &mut App| {};

    // Run App
    app.run(init, update, render, exit);
}
