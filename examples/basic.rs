use lunar::app::App;

fn main() {
    // Create a lunar app
    let app = App::new();

    // Initialize App
    let init = move |_app: &mut App| {};

    // Update App
    let update = move |_app: &mut App| {};

    // Render objects
    let render = move |_app: &mut App| {};

    // Exit App
    let exit = move |_app: &mut App| {};

    // Run App
    app.run(init, update, render, exit);
}
