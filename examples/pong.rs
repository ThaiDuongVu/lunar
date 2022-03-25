use lunar::app::App;
use lunar::types::vector2int::Vector2Int;

const WIDTH: i32 = 120;
const HEIGHT: i32 = 30;

fn main() {
    // Create a lunar app
    let app = App::new();

    // Initialize App
    let init = move |app: &mut App| {
        app.set_size(Vector2Int {
            x: WIDTH,
            y: HEIGHT,
        });
        app.set_all_borders('*');
        app.set_all_corners('*');
    };

    // Update App
    let update = move |_app: &mut App| {};

    // Render objects
    let render = move |_app: &mut App| {};

    // Exit App
    let exit = move |_app: &mut App| {};

    // Run App
    app.run(init, update, render, exit);
}
