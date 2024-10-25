use lunar::app::App;

fn main() {
    // Create a lunar app
    let app = App::new();

    // Initialize App
    let init = |app: &mut App| {
        app.set_all_borders('*');
        app.set_all_corners('*');
    };

    // Update App
    let update = |_app: &mut App| {};

    // Render objects
    let render = |_app: &mut App| {};

    // Exit App
    let exit = |_app: &mut App| {};

    // Run App
    app.run(init, update, render, exit);
}
