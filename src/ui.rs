use gtk::{ContainerExt, GtkWindowExt, WidgetExt};

pub mod accelerators;
pub mod actions;
pub mod widgets;

use accelerators::add_accelerators;
use actions::add_actions;
use widgets::{build_content, build_system_menu};


pub fn build(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Kaati Ako");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);
    window.add(&build_content(&window));
    build_system_menu(application);
    add_accelerators(application);
    add_actions(application, &window);
    window.show_all();
}
