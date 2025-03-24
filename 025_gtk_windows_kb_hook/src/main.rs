use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextView, TextBuffer, Button, Box, Orientation};
use gdk::keys::constants as key;
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
    let line_number = Rc::new(RefCell::new(0));

    // Function to add log line
    let add_log_line = move |log_data: &Rc<RefCell<(TextBuffer, Vec<String>)>>, line_number: &Rc<RefCell<usize>>, message: String| {
        let (ref text_buffer, ref mut log_lines) = *log_data.borrow_mut();
        if log_lines.len() >= 10 {
            log_lines.remove(0);
        }
        log_lines.push(format!("{} {}", message, *line_number));
        *line_number = (*line_number + 1) % 10;
        text_buffer.set_text(&log_lines.join("\n"));
    };

    // Connect button click to add log line
    let log_data_clone = Rc::clone(&log_data);
    let line_number_clone = Rc::clone(&line_number);
    let add_log_line_clone = add_log_line.clone();
    button.connect_clicked(move |_| {
        add_log_line_clone(&log_data_clone, &line_number_clone, "New log line".to_string());
    });

    // Capture keyboard events
    let log_data_clone = Rc::clone(&log_data);
    let line_number_clone = Rc::clone(&line_number);
    let add_log_line_clone = add_log_line.clone();
    window.connect_key_press_event(move |_, event| {
        let keyval = event.keyval();
        let state = event.state();

        if state.contains(gdk::ModifierType::CONTROL_MASK) && keyval == key::A {
            add_log_line_clone(&log_data_clone, &line_number_clone, "Ctrl+A pressed".to_string());
        } else if state.contains(gdk::ModifierType::SHIFT_MASK) && keyval == key::A {
            add_log_line_clone(&log_data_clone, &line_number_clone, "Shift+A pressed".to_string());
        } else if keyval == key::Escape {
            add_log_line_clone(&log_data_clone, &line_number_clone, "Escape pressed".to_string());
        }

        Inhibit(false)
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
