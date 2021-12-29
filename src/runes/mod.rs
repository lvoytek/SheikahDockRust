mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct RunesWidget(ObjectSubclass<imp::RunesWidget>) @extends gtk::Widget;
}

impl Default for RunesWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl RunesWidget {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a Sheikah Launcher Widget")
    }
}
