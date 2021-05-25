//! Build the application widgets

use std::fs;

use glib;
use gtk::{AboutDialogExt, ActionBarExt, BoxExt, ButtonExt, GtkApplicationExt, GtkWindowExt, WidgetExt, prelude::NotebookExtManual, LabelExt};

use crate::util::database::Card;
use crate::{DB_CONNECTION, VERSION};
use super::actions::{action_next_card};


/// The name of content widget which contains the flash card
pub const WIDGET_NAME_CONTENT: &str = "content";
/// The name of flash card's widget
pub const WIDGET_NAME_CARD: &str = "card";


/// Build a flash card as Notebook widget
pub fn build_card() -> gtk::Notebook {
    let mut card = Card::get_empty();
    DB_CONNECTION.with(|cell| {
        card = match cell.borrow().as_ref() {
            Some(conn) => Card::get_random(&conn),
            None => Card::get_empty(),
        };
    });
    let padding = 10;
    let notebook = gtk::Notebook::new();
    notebook.set_widget_name(WIDGET_NAME_CARD);
    notebook.grab_focus();
    for translation in card.translations {
        let page = gtk::Box::new(gtk::Orientation::Vertical, 0);
        page.set_homogeneous(false);

        let text = gtk::Label::new(Some(""));
        text.set_markup(&format!("<span font_desc='30.0'>{}</span>", &translation.text));
        page.pack_start(&text, true, true, padding);

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        page.pack_start(&separator, false, false, padding);

        let page_bottom = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        if translation.description != "" {
            let description = gtk::Label::new(Some(&translation.description));
            page_bottom.pack_start(&description, false, false, padding);
        }
        let category = gtk::Label::new(Some(""));
        category.set_markup(&format!("Category: <b>{}</b>", card.category.name));
        page_bottom.pack_end(&category, false, false, padding);
        page.pack_start(&page_bottom, false, false, padding);

        let tab_label = gtk::Label::new(Some(&translation.language.name));
        notebook.append_page(&page, Some(&tab_label));
    }
    notebook
}


/// Build the application's action bar
fn build_action_bar(window: &gtk::ApplicationWindow) -> gtk::ActionBar {
    let action_bar = gtk::ActionBar::new();
    let next = gtk::Button::from_icon_name(
        Some("go-next"), gtk::IconSize::Button);
    next.connect_clicked(glib::clone!(@weak window => move |_| {
        action_next_card(&window);
    }));
    action_bar.pack_start(&next);
    let label = gtk::Label::new(Some("Press button or type <n> for next random card."));
    action_bar.pack_start(&label);
    action_bar
}


/// Build the application's content area with flash card and action bar
pub fn build_content(window: &gtk::ApplicationWindow) -> gtk::Box {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.set_widget_name(WIDGET_NAME_CONTENT);
    let card = build_card();
    content.pack_start(&card, true, true, 10);
    let action_bar = build_action_bar(window);
    content.pack_end(&action_bar, false, false, 0);
    content
}


/// Build the application's system menu
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

/// Show an about dialog
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
