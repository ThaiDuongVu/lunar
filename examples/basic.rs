use lunar::app::App;
use lunar::types::mouse_button::MouseButton;

fn main() {
    // Create a lunar app
    let app = App::new();

    // Initialize App
    let init = move |app: &mut App| {
        app.set_all_borders_corners('*');
    };

    // Update App
    let update = move |app: &mut App| {
        if app.input.on_mouse_clicked(MouseButton::Left) {
            println!("left mouse down");
        }
        if app.input.is_character_down('q') {
            app.quit();
        }
    };

    // Render objects
    let render = move |_app: &mut App| {};

    // Exit App
    let exit = move |_app: &mut App| {};

    // Run App
    app.run(init, update, render, exit);
}
