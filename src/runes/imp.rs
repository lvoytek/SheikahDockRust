use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene};

#[derive(Default)]
pub struct RunesWidget {}

#[glib::object_subclass]
impl ObjectSubclass for RunesWidget {
    const NAME: &'static str = "RunesWidget";
    type Type = super::RunesWidget;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for RunesWidget {}

impl WidgetImpl for RunesWidget {
    fn measure(
        &self,
        _widget: &Self::Type,
        _orientation: gtk::Orientation,
        _for_size: i32,
    ) -> (i32, i32, i32, i32) {
        // We need some space to draw
        (100, 200, -1, -1)
    }

    fn snapshot(&self, widget: &Self::Type, snapshot: &gtk::Snapshot) {
        // Draw a square in each launcher location
        // By default there are six squares taking up the center half of the screen
        let window_width = (widget.width()) as f32;
        let window_height = (widget.height()) as f32;

        // TODO: Make number of runes and spacing dynamic
        let rune_spacing: f32 = 5.0;
        let num_runes: u8 = 6;

        let rune_size = (window_width / 2.0) / (num_runes as f32) - rune_spacing;
        let start_location_x = (window_width / 2.0) - (((num_runes as f32) / 2.0) * rune_size) - ((((num_runes as f32) / 2.0) - 0.5) * rune_spacing);
        let start_location_y = (window_height / 2.0) - (0.5 * rune_size);

        let green_color = gdk::RGBA::green();

        for i in 0..num_runes {
            let next_x = start_location_x + ((i as f32) * (rune_size + rune_spacing));
            let rect = graphene::Rect::new(next_x, start_location_y, rune_size, rune_size);
            snapshot.append_color(&green_color, &rect);
        }
    }
}
