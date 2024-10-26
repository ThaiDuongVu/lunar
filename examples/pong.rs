use lunar::app::App;

fn main() {
    // Create a lunar app
    let app = App::new();

    // Initialize App
    let init = |app: &mut App| {
        app.set_title("Pong".to_string());
    };

    // Update App
    let update = |app: &mut App| {
        if app.input.is_char_key_down('q') {
            app.quit();
        }
    };

    // Render objects
    let render = |_app: &mut App| {};

    // Exit App
    let exit = |_app: &mut App| {};

    // Run App
    app.run(init, update, render, exit);
}
