use std::cell::Cell;

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ButtonExt;

// Object holding the state
// In this struct, we setted a "number" that accompanies the button. So every button will have a internal number variable. 
#[derive(Default)]
pub struct CustomButton {
    number: Cell<i32>
}

// The central trait for subclassing a GObject
// This is a custom GObject based in the button. 

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButton {}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
// Overriding the function of clicked, the button will add +1 to the counter of the variable "number"
// And automatically changes the label
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        self.number.set(self.number.get() + 1);
        self.obj().set_label(&self.number.get().to_string())
    }
}