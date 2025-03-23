use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextView, TextBuffer, Button, Box, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

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
    window.set_default_size(350, 200);
    window.set_position(gtk::WindowPosition::Center);

    // Create a vertical box to hold the text view and button
    let vbox = Box::new(Orientation::Vertical, 5);

    // Create a new text view
    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);

    // Create a new text buffer
    let text_buffer = TextBuffer::new(None::<&gtk::TextTagTable>);

    // Set the text buffer to the text view
    text_view.set_buffer(Some(&text_buffer));

    // Create a button to add log lines
    let button = Button::with_label("Add Log Line");

    // Create a reference-counted cell to hold the text buffer and log lines
    let log_data = Rc::new(RefCell::new((text_buffer, Vec::new())));

    // Connect button click to add log line
    let log_data_clone = Rc::clone(&log_data);
    button.connect_clicked(move |_| {
        let (ref text_buffer, ref mut log_lines) = *log_data_clone.borrow_mut();
        if log_lines.len() >= 10 {
            log_lines.remove(0);
        }
        log_lines.push("New log line".to_string());
        text_buffer.set_text(&log_lines.join("\n"));
    });

    // Add the text view and button to the vertical box
    vbox.pack_start(&text_view, true, true, 0);
    vbox.pack_start(&button, false, false, 0);

    // Add the vertical box to the window
    window.add(&vbox);

    // Show the window and its contents
    window.show_all();

    // Put the window in front
    window.present();

    // Force the window to stay on top
    window.set_keep_above(true);
}
