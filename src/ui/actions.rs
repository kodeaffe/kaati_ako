use gio::ActionMapExt;
use glib::Cast;
use gtk::{BoxExt, ContainerExt, GtkWindowExt, WidgetExt};

use super::widgets::{WIDGET_NAME_CARD, WIDGET_NAME_CARD_BOX, build_card, show_about};


pub fn action_next_card(window: &gtk::ApplicationWindow) {
    // TODO: Is there a better way to find the box and card?
    for widget in window.get_children() {
        if widget.get_widget_name() == WIDGET_NAME_CARD_BOX {
            match widget.downcast::<gtk::Box>() {
                Ok(vbox) => {
                    for child in vbox.get_children() {
                        if child.get_widget_name() == WIDGET_NAME_CARD {
                            match child.downcast::<gtk::Notebook>() {
                                Ok(card) => {
                                    vbox.remove(&card);
                                    let card = build_card();
                                    vbox.pack_start(&card, true, true, 10);
                                    vbox.show_all();
                                    card.grab_focus(); // Focus must be grabbed after being shown
                                    return;
                                },
                                _ => {},
                            }
                        }
                    }
                },
                _ => {},
            }
        }
    }
}


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

    let next_card = gio::SimpleAction::new("next_card", None);
    next_card.connect_activate(glib::clone!(@weak window => move |_, _| {
        action_next_card(&window);
    }));
    application.add_action(&next_card);
}