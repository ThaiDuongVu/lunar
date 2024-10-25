use lunar::{app::App, input::SpecialKey};

fn main() {
    // Create a lunar app
    let app = App::new();

    // Initialize App
    let init = |app: &mut App| {
        app.set_all_borders('*');
        app.set_all_corners('*');
    };

    // Update App
    let update = |app: &mut App| {
        let char_key = app.input.char_key_down;
        if char_key == 'q' {
            app.quit();
        } else if char_key != ' ' {
            app.set_all_borders(char_key);
            app.set_all_corners(char_key);
        }

        let special_key = app.input.special_key_down;
        if special_key == SpecialKey::Up {
            println!("up!");
        }
    };

    // Render objects
    let render = |_app: &mut App| {};

    // Exit App
    let exit = |_app: &mut App| {};

    // Run App
    app.run(init, update, render, exit);
}
