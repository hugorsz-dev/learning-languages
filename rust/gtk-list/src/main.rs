
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, ListBox, Label, PolicyType, ScrolledWindow};

const APP_ID: &str = "org.gtk_rs.GObjectList";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}


fn build_ui(app: &Application) {
    // Create a `ListBox` and add labels with integers from 0 to 100
    let list_box = ListBox::new();

    for number in 0..=100 {
        let label = Label::new(Some((&number.to_string())));
        list_box.append(&label);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_box)
        .build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    // Present window
    window.present();
}