// main.rs

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

fn main() {
    // Initialize GTK application
    let app = Application::new(Some("com.example.gtk-rs-app"), Default::default());

    // Connect activate signal to create window and label
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &gtk::Application) {
    // Create a new window
    let window = ApplicationWindow::new(app);
    window.set_title("Rust Desktop App");
    window.set_default_size(300, 200);

    // Create a new label
    let label = Label::new(Some("Hello, World!"));

    // Add the label to the window
    window.add(&label);

    // Show the window
    window.show_all();
}
