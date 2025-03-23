// main.rs

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextView, TextBuffer};

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
    window.set_title("Wow");
    window.set_default_size(350, 100);
    window.set_position(gtk::WindowPosition::Center);

    // Create a new text view
    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);

    // Create a new text buffer
    let text_buffer = TextBuffer::new(None::<&gtk::TextTagTable>);

    // Set the text to the text buffer
    text_buffer.set_text("Hello, World!");

    // Set the text buffer to the text view
    text_view.set_buffer(Some(&text_buffer));

    // Add the text view to the window
    window.add(&text_view);

    // Show the window and its contents
    window.show_all();

    // Put the window in front
    window.present();

    // Force the window to stay on top
    window.set_keep_above(true);
}
