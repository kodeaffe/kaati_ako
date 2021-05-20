use std::env::args;

use gio::prelude::*;
use glib;
use gtk::prelude::*;

mod util;
use util::database::{connect_database, create_database, get_random_card};


fn add_accelerators(application: &gtk::Application) {
    // `Primary` is a platform-agnostic accelerator modifier.
    // On Windows and Linux, `Primary` maps to the `Ctrl` key,
    // and on macOS it maps to the `command` key.
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
}


fn add_actions(application: &gtk::Application, window: &gtk::ApplicationWindow) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));
    application.add_action(&quit);
}

fn build_card() -> gtk::Notebook {
    let conn = connect_database();
    let card = get_random_card(&conn);
    let notebook = gtk::Notebook::new();
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
}


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Kaati Ako");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);

    let card = build_card();
    vbox.pack_start(&card, true, true, 10);

    let action_bar = gtk::ActionBar::new();
    let next = gtk::Button::from_icon_name(
        Some("go-next"), gtk::IconSize::Button);
    next.connect_clicked(glib::clone!(@weak vbox => move |_| {
        for child in vbox.get_children() {
            match child.downcast::<gtk::Notebook>() {
                Ok(card) => {
                    vbox.remove(&card);
                    let card = build_card();
                    vbox.pack_start(&card, true, true, 10);
                    vbox.show_all();
                    break;
                },
                _ => {},
            }
        }
    }));
    action_bar.pack_start(&next);
    vbox.pack_end(&action_bar, false, false, 0);

    build_system_menu(application);
    add_actions(application, &window);
    window.show_all();
}


fn main() {
    let conn = connect_database();
    create_database(&conn);

    let application = gtk::Application::new(
            Some("com.github.kodeaffe.kaati_ako"), Default::default()).unwrap();
    application.connect_startup(|app| {
        add_accelerators(app);
    });
    application.connect_activate(build_ui);
    application.run(&args().collect::<Vec<_>>());
}