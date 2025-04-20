mod imp;

use glib::Object;
use gtk::glib;

// Wrapper that is mandatory because of the C base. 

glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

// This is a GObject. Is the only type of variable that allows the scalable list (ListStore)

impl IntegerObject {
    pub fn new(number: i32) -> Self {
        Object::builder().property("number", number).build()
    }
}