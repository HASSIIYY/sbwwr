// Connecting modules
use fltk::{prelude::*, *};

fn main() {
    // App/Window constructor
    let app = app::App::default();
    let mut window = window::Window::default()
        .with_size(800, 600)
        .center_screen();


    // Objects


    // Window end/show
    window.end();
    window.show();


    // Launching app
    app.run().unwrap();
}
