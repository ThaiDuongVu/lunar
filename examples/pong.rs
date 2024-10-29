use lunar::{
    app::{App, Color},
    entities::game_object::GameObject,
    input::SpecialKey,
    types::vector2int::Vector2Int,
};
use ndarray::array;
use rand::Rng;
use std::cell::Cell;

const BALL_SPEED: i32 = 1;
const PLAYER_SPEED: i32 = 1;

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

    let mut rng = rand::thread_rng();
    let ball_direction = Cell::new(Vector2Int {
        x: -1,
        y: rng.gen_range(-1..2),
    });
    let ball_map = array![['o']];
    let ball = Cell::new(GameObject::new(&ball_map));

    // Initialize App
    let init = |app: &mut App| {
        app.set_width(101);
        app.set_height(29);
        app.set_title("Pong".to_string());
        app.set_background_color(Color::Black);

        divider.set(divider.get().move_to(Vector2Int { x: 50, y: 0 }));
        player1.set(player1.get().move_to(Vector2Int { x: 3, y: 12 }));
        player2.set(player2.get().move_to(Vector2Int { x: 97, y: 12 }));
        ball.set(ball.get().move_to(Vector2Int { x: 50, y: 14 }));
    };

    // Update App
    let update = |app: &mut App| {
        if app.input.is_char_key_down('q') {
            app.quit();
        }

        // Move player 1 using W/S keys
        if app.input.is_char_key_down('w') && player1.get().get_position().y > 1 {
            player1.set(player1.get().move_by(Vector2Int::down() * PLAYER_SPEED));
        }
        if app.input.is_char_key_down('s') && player1.get().get_position().y < 23 {
            player1.set(player1.get().move_by(Vector2Int::up() * PLAYER_SPEED));
        }

        // Move player 2 using arrow keys
        if app.input.is_special_key_down(SpecialKey::Up) && player2.get().get_position().y > 1 {
            player2.set(player2.get().move_by(Vector2Int::down() * PLAYER_SPEED));
        }
        if app.input.is_special_key_down(SpecialKey::Down) && player2.get().get_position().y < 23 {
            player2.set(player2.get().move_by(Vector2Int::up() * PLAYER_SPEED));
        }

        // Move ball
        ball.set(ball.get().move_by(ball_direction.get() * BALL_SPEED));
        // Handle ball bounce
        if ball.get().get_position().y <= 1 {
            ball_direction.set(Vector2Int {
                x: rng.gen_range(-1..2),
                y: 1,
            });
        } else if ball.get().get_position().y >= 28 {
            ball_direction.set(Vector2Int {
                x: rng.gen_range(-1..2),
                y: -1,
            });
        }
        if ball.get().get_position().x <= 1 {
            ball_direction.set(Vector2Int {
                x: 1,
                y: rng.gen_range(-1..2),
            });
        } else if ball.get().get_position().x >= 100 {
            ball_direction.set(Vector2Int {
                x: -1,
                y: rng.gen_range(-1..2),
            });
        }
    };

    // Render objects on App window
    let render = |app: &mut App| {
        divider.get().render(app);
        player1.get().render(app);
        player2.get().render(app);
        ball.get().render(app);
    };

    // On App exit
    let exit = |_app: &mut App| {};

    // Run App
    app.run(init, update, render, exit);
}
