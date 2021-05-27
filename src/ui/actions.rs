//! Handle application actions

use gio::ActionMapExt;
use gtk::GtkWindowExt;

use super::widgets::{replace_card, show_about, show_add_card};


/// Add actions for the application
pub fn add_actions(application: &gtk::Application, window: &gtk::ApplicationWindow) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));
    application.add_action(&quit);

    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(glib::clone!(@weak window => move |_, _| {
        show_about(&window);
    }));
    application.add_action(&about);

    let add_card = gio::SimpleAction::new("add_card", None);
    add_card.connect_activate(glib::clone!(@weak window => move |_, _| {
        show_add_card(&window);
    }));
    application.add_action(&add_card);

    let next_card = gio::SimpleAction::new("next_card", None);
    next_card.connect_activate(glib::clone!(@weak window => move |_, _| {
        replace_card(&window, 0);
    }));
    application.add_action(&next_card);
}