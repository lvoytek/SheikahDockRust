mod imp;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct RuneWidget(ObjectSubclass<imp::RuneWidget>) @extends gtk::Widget, gtk::Button;
}

impl Default for RuneWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl RuneWidget {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a Rune Widget")
    }

    pub fn set_app_info(&self, app_info: &gio::AppInfo) {
        let self_ = imp::RuneWidget::from_instance(self);

        if let Some(icon) = app_info.icon() {
            self_.image.set_from_gicon(&icon);
        }

        self_.background.set_from_file(Some("assets/rune_background.svg"));
    }
}
