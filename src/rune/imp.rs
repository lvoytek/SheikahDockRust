use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::CompositeTemplate;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "rune.ui")]
pub struct RuneWidget {
    #[template_child]
    pub background: TemplateChild<gtk::Image>,
    #[template_child]
    pub image: TemplateChild<gtk::Image>,
    #[template_child]
    pub icons: TemplateChild<gtk::Box>
}

#[glib::object_subclass]
impl ObjectSubclass for RuneWidget {
    const NAME: &'static str = "RuneWidget";
    type Type = super::RuneWidget;
    type ParentType = gtk::Button;

    fn class_init(class_var: &mut Self::Class) {
        Self::bind_template(class_var);
        class_var.set_layout_manager_type::<gtk::BinLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RuneWidget {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        self.icons.set_layout_manager(Some(&gtk::BinLayout::new()));

        // Connect a gesture to handle clicks.
        let gesture = gtk::GestureClick::new();
        gesture.connect_released(|gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
        });
        obj.add_controller(&gesture);
    }
}

impl WidgetImpl for RuneWidget {}
impl BoxImpl for RuneWidget {}
impl ButtonImpl for RuneWidget {}
