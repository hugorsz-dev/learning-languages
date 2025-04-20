use gtk::{prelude::*, Label};
use gtk::{self, glib, Application, ApplicationWindow, Button, Box, Orientation};
use std::cell::Cell;
use crate::glib::clone;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement0";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(application: &Application) {

    
    
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .build();
    let text_label = Label::builder().label("prueba");

    let button_box = Box::new(Orientation::Vertical, 5);
    button_box.append(&button_increase);
    button_box.append(&button_decrease);


    // A mutable integer
    // -> Because 'number' only exists in build_ui function, that is not static, it needs the modifier "move" before the closure and a cell
    // let mut number = Cell::new(0); -> If only is intended to modify by one only button, the type cell is the best choice.
    
    // Rc is a reference, so it applies to all types of variables, included buttons or any widgets. 
    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    // The use of clone! strong number implies a "hard referenced variable", that means it is cloned and it persists
    // When this is done - like in button_increase - , is recommended to put rest of references as weak
    // Because if both are hard refernces, they do not expire. 
    
    button_increase.connect_clicked(clone! (#[strong] number, #[strong] button_decrease, move |button_increase: &Button| {
        number.set(number.get() + 1);
        button_increase.set_label(&format!("({}) Increase", &number.get()));
        button_decrease.set_label(&format!("({}) Decrease", &number.get()));

    }));

    button_decrease.connect_clicked(clone! (#[weak] number, #[weak] button_increase, move |button_decrease: &Button| {
        number.set(number.get() - 1);
        button_decrease.set_label(&format!("({}) Decrease", &number.get()));
        button_increase.set_label(&format!("({}) Increase", &number.get()));
        
    }));

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&button_box)
        .build();

    // Present the window
    window.present();
}