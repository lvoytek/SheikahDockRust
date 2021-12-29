mod runes;

use gtk::prelude::*;
use runes::RunesWidget;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.lvoytek.sheikah_dock"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::Window::builder()
        .default_width(800)
        .default_height(600)
        .application(app)
        .decorated(false)
        .maximized(true)
        //.opacity(0.0)
        .title("Sheikah Launcher")
        .build();

    let widget = RunesWidget::new();
    window.set_child(Some(&widget));
    window.show();
}
