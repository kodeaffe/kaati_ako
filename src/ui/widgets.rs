use std::fs;

use glib;
use gtk::{
    AboutDialogExt,
    ActionBarExt,
    BoxExt,
    ButtonExt,
    GtkApplicationExt,
    GtkWindowExt,
    WidgetExt,
    prelude::NotebookExtManual,
};

use crate::util::database::{connect_database, get_random_card};
use crate::VERSION;
use super::actions::{action_next_card};


pub const WIDGET_NAME_CARD_BOX: &str = "card_box";
pub const WIDGET_NAME_CARD: &str = "card";


pub fn build_card() -> gtk::Notebook {
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


fn build_action_bar(window: &gtk::ApplicationWindow) -> gtk::ActionBar {
    let action_bar = gtk::ActionBar::new();
    let next = gtk::Button::from_icon_name(
        Some("go-next"), gtk::IconSize::Button);
    next.connect_clicked(glib::clone!(@weak window => move |_| {
        action_next_card(&window);
    }));
    action_bar.pack_start(&next);
    let label = gtk::Label::new(Some("Press button or 'n' for next random card."));
    action_bar.pack_start(&label);
    action_bar
}


pub fn build_content(window: &gtk::ApplicationWindow) -> gtk::Box {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.set_widget_name(WIDGET_NAME_CARD_BOX);
    let card = build_card();
    content.pack_start(&card, true, true, 10);
    let action_bar = build_action_bar(window);
    content.pack_end(&action_bar, false, false, 0);
    content
}


pub fn build_system_menu(application: &gtk::Application) {
    let menu = gio::Menu::new();
    menu.append(Some("Quit"), Some("app.quit"));
    application.set_app_menu(Some(&menu));

    let menu_bar = gio::Menu::new();
    let more_menu = gio::Menu::new();
    more_menu.append(Some("About"), Some("app.about"));
    menu_bar.append_submenu(Some("?"), &more_menu);
    application.set_menubar(Some(&menu_bar));
}


pub fn show_about(window: &gtk::ApplicationWindow) {
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
    dialog.set_transient_for(Some(window));
    dialog.set_version(Some(VERSION));
    dialog.show_all();
}