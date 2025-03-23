// main.rs

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextView, TextBuffer};
use winapi::shared::windef::HWND;

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
    let wow_handles = get_wow_handles();
    text_buffer.set_text(&wow_handles);

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

fn get_wow_handles() -> String {
    let fen_title = "World of Warcraft";
    let window_list = gtk::Window::list_toplevels();

    if window_list.len() == 0 {
        return "Aucune fenêtre de World of Warcraft trouvée".to_string();
    } else if window_list.len() == 1 {
        let window = &window_list[0];
        let title = window.downcast_ref::<gtk::Window>().unwrap().get_title();
        let allocation = window.get_allocation();
        let width = allocation.width;
        let height = allocation.height;
        let x = allocation.x;
        let y = allocation.y;

        format!("Une seule fenêtre de World of Warcraft trouvée :\n\
                 Titre : {}\n\
                 Largeur : {}\n\
                 Hauteur : {}\n\
                 Position : ({}, {})", title, width, height, x, y)
    } else if window_list.len() < 2 {
        return "No Double Wow Windows".to_string();
    } else {
        // Vous devez trouver une autre façon d'obtenir le handle de la fenêtre
        // let wow_hwnd1 = window_list[0].get_window().unwrap() as *mut gdk::Window;
        // let wow_hwnd2 = window_list[1].get_window().unwrap() as *mut gdk::Window;

        format!("Wow Hwnd1: ?, Wow Hwnd2: ?")
    }
}
