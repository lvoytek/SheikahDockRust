mod rune;

use gtk::gio;
use gtk::prelude::*;
use rune::RuneWidget;

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
        .title("Sheikah Launcher")
        .build();

    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    let test_app = gio::AppInfo::all()[11].clone();
    let context = grid.display().app_launch_context().clone();

    let test_rune = RuneWidget::new();
    test_rune.set_app_info(&test_app);

    let width = 800;

    let auto_size = (width / 6) - 12;
    test_rune.scale_to_size(auto_size);

    test_rune.connect_clicked(move |rune| {
        if let Err(err) = test_app.launch(&[], Some(&context)) {
            let parent_window = rune.root().unwrap().downcast::<gtk::Window>().unwrap();

            gtk::MessageDialog::builder()
                .text(&format!("Failed to start {}", test_app.name()))
                .secondary_text(&err.to_string())
                .message_type(gtk::MessageType::Error)
                .modal(true)
                .transient_for(&parent_window)
                .build()
                .show();
        }
    });

    grid.attach(&test_rune, 0, 0, 1, 1);

    // Add the grid in the window
    window.set_child(Some(&grid));
    window.show();
}
