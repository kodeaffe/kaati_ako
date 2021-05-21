use std::fs;
use std::env::args;

use gio::prelude::*;
use glib;
use gtk::prelude::*;

mod util;
use util::database::{connect_database, get_random_card};


const VERSION: &str = "0.1.0";
const WIDGET_NAME_CARD_BOX: &str = "card_box";
const WIDGET_NAME_CARD: &str = "card";


fn add_accelerators(application: &gtk::Application) {
    application.set_accels_for_action("app.about", &["F1"]);
    application.set_accels_for_action("app.next_card", &["n"]);
    // `Primary` is a platform-agnostic accelerator modifier.
    // On Windows and Linux, `Primary` maps to the `Ctrl` key,
    // and on macOS it maps to the `command` key.
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
}

fn action_next_card(window: &gtk::ApplicationWindow) {
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


fn add_actions(application: &gtk::Application, window: &gtk::ApplicationWindow) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));
    application.add_action(&quit);

    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(glib::clone!(@weak window => move |_, _| {
        let dialog = gtk::AboutDialog::new();
        dialog.set_authors(&["kodeaffe"]);
        let licence = fs::read_to_string("LICENSE").unwrap();
        dialog.set_comments(Some("This application will hopefully help in learning a language."));
        dialog.set_copyright(Some("All rights reversed"));
        dialog.set_license(Some(&licence));
        dialog.set_logo_icon_name(Some("accessories-dictionary"));
        dialog.set_program_name("kaati ako");
        dialog.set_website_label(Some("kaati ako on github"));
        dialog.set_website(Some("https://github.com/kodeaffe/kaati_ako"));
        dialog.set_title("About");
        dialog.set_transient_for(Some(&window));
        dialog.set_version(Some(VERSION));
        dialog.show_all();
    }));
    application.add_action(&about);

    let next_card = gio::SimpleAction::new("next_card", None);
    next_card.connect_activate(glib::clone!(@weak window => move |_, _| {
        action_next_card(&window);
    }));
    application.add_action(&next_card);
}

fn build_card() -> gtk::Notebook {
    let conn = connect_database();
    let card = get_random_card(&conn);
    let notebook = gtk::Notebook::new();
    notebook.set_widget_name(WIDGET_NAME_CARD);
    for translation in card.translations {
        let page = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let text = gtk::Label::new(Some(&translation.text));
        page.pack_start(&text, true, true, 10);
        if translation.description != "" {
            let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
            page.pack_start(&separator, false, false, 10);
            let description = gtk::Label::new(Some(&translation.description));
            page.pack_start(&description, true, true, 10);
        }
        let tab_label = gtk::Label::new(Some(&translation.language.name));
        notebook.append_page(&page, Some(&tab_label));
    }
    notebook
}


fn build_system_menu(application: &gtk::Application) {
    let menu = gio::Menu::new();
    menu.append(Some("Quit"), Some("app.quit"));
    application.set_app_menu(Some(&menu));

    let menu_bar = gio::Menu::new();
    let more_menu = gio::Menu::new();
    more_menu.append(Some("About"), Some("app.about"));
    menu_bar.append_submenu(Some("?"), &more_menu);
    application.set_menubar(Some(&menu_bar));

}


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Kaati Ako");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.set_widget_name(WIDGET_NAME_CARD_BOX);
    window.add(&vbox);

    let card = build_card();
    vbox.pack_start(&card, true, true, 10);

    let action_bar = gtk::ActionBar::new();
    let next = gtk::Button::from_icon_name(
        Some("go-next"), gtk::IconSize::Button);
    next.connect_clicked(glib::clone!(@weak window => move |_| {
        action_next_card(&window);
    }));
    action_bar.pack_start(&next);
    let label = gtk::Label::new(Some("Press button or 'n' for next random card."));
    action_bar.pack_start(&label);
    vbox.pack_end(&action_bar, false, false, 0);

    build_system_menu(application);
    add_accelerators(application);
    add_actions(application, &window);
    window.show_all();
}


fn main() {
    /*
    let conn = connect_database();
    create_database(&conn);
    */

    let application = gtk::Application::new(
        Some("com.github.kodeaffe.kaati_ako"), Default::default()).unwrap();
    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}