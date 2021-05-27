//! Build the application widgets

use std::fs;

use glib;
use glib::Cast;
use gtk::{
    AboutDialogExt,
    ActionBarExt,
    BoxExt,
    ButtonExt,
    ContainerExt,
    DialogExt,
    EntryExt,
    GridExt,
    GtkApplicationExt,
    GtkWindowExt,
    LabelExt,
    WidgetExt,
    prelude::NotebookExtManual,
};

use crate::models::card::Card;
use crate::database::get_connection;
use crate::VERSION;


/// The name of content widget which contains the flash card
pub const WIDGET_NAME_CONTENT: &str = "content";
/// The name of flash card's widget
pub const WIDGET_NAME_CARD: &str = "card";


/// Build the application's action bar
fn build_action_bar(window: &gtk::ApplicationWindow) -> gtk::ActionBar {
    let action_bar = gtk::ActionBar::new();
    let next = gtk::Button::from_icon_name(
        Some("go-next"), gtk::IconSize::Button);
    next.connect_clicked(glib::clone!(@weak window => move |_| {
        replace_card(&window, 0);
    }));
    action_bar.pack_start(&next);
    let label = gtk::Label::new(Some("Press button or type <n> for next random card."));
    action_bar.pack_start(&label);
    action_bar
}


/// Build a flash card as Notebook widget, uses a random card if given card_id has value 0
pub fn build_card(window: &gtk::ApplicationWindow, card_id: i64) -> gtk::Notebook {
    let notebook = gtk::Notebook::new();
    notebook.set_widget_name(WIDGET_NAME_CARD);
    notebook.grab_focus();

    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(err) => {
            show_error(window, &err.to_string());
            return notebook;
        }
    };
    let mut id = card_id;
    if id == 0 {
        id = match Card::random_id(&conn) {
            Ok(id) => id,
            Err(err) => {
                show_error(window, &err.to_string());
                return notebook;
            }
        }
    }
    let card = match Card::get(&conn, id) {
        Ok(card) => card,
        Err(err) => {
            show_error(window, &err.to_string());
            Card::from_empty()
        }
    };

    let padding = 10;
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


/// Build the application's content area with flash card and action bar
pub fn build_content(window: &gtk::ApplicationWindow) -> gtk::Box {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.set_widget_name(WIDGET_NAME_CONTENT);

    let card = build_card(window, 0);
    content.pack_start(&card, true, true, 10);

    let action_bar = build_action_bar(window);
    content.pack_end(&action_bar, false, false, 0);
    content
}


/// Build the application's menu
pub fn build_menu(application: &gtk::Application) {
    let menu = gio::Menu::new();
    menu.append(Some("Quit"), Some("app.quit"));
    application.set_app_menu(Some(&menu));

    let card_menu = gio::Menu::new();
    card_menu.append(Some("Add card"), Some("app.add_card"));
    card_menu.append(Some("Edit current card"), Some("app.edit_card"));

    let about_menu = gio::Menu::new();
    about_menu.append(Some("About"), Some("app.about"));

    let menu_bar = gio::Menu::new();
    menu_bar.append_submenu(Some("Card"), &card_menu);
    menu_bar.append_submenu(Some("?"), &about_menu);
    application.set_menubar(Some(&menu_bar));
}


/// Replace the shown flash card by the card with given id
pub fn replace_card(window: &gtk::ApplicationWindow, card_id: i64) {
    // TODO: Is there a better way to find the box and card?
    for widget in window.get_children() {
        if widget.get_widget_name() == WIDGET_NAME_CONTENT {
            match widget.downcast::<gtk::Box>() {
                Ok(vbox) => {
                    for child in vbox.get_children() {
                        if child.get_widget_name() == WIDGET_NAME_CARD {
                            match child.downcast::<gtk::Notebook>() {
                                Ok(card) => {
                                    vbox.remove(&card);
                                    let card = build_card(window, card_id);
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


/// Show an about dialog
pub fn show_about(parent: &gtk::ApplicationWindow) {
    let dialog = gtk::AboutDialog::new();
    dialog.set_authors(&["kodeaffe <lahi+kodeaffe@posteo.de>"]);
    let licence_path = "LICENSE";
    let licence = match fs::read_to_string(licence_path) {
        Ok(licence) => licence,
        Err(err) => {
            show_error(parent, &format!("{}: {}", err, licence_path));
            return;
        },
    };
    dialog.set_comments(Some("This application will hopefully help in learning a language."));
    dialog.set_copyright(Some("All rights reversed"));
    dialog.set_license(Some(&licence));
    dialog.set_logo_icon_name(Some("accessories-dictionary"));
    dialog.set_program_name("kaati ako");
    dialog.set_website_label(Some("kaati ako on github"));
    dialog.set_website(Some("https://github.com/kodeaffe/kaati_ako"));
    dialog.set_title("About");
    dialog.set_transient_for(Some(parent));
    dialog.set_version(Some(VERSION));
    dialog.show_all();
}


/// Show a dialog to add a new flash card
pub fn show_add_card(parent: &gtk::ApplicationWindow) {
    let dialog = gtk::Dialog::with_buttons(
        Some("Add Card"),
        Some(parent),
        gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
        &[
            ("_Ok", gtk::ResponseType::Accept),
            ("_Cancel", gtk::ResponseType::Cancel),
        ],
    );

    let content = dialog.get_content_area();
    let text = gtk::Label::new(Some("Category: default"));
    content.pack_start(&text, true, true, 20);

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5);
    grid.set_row_spacing(10);
    let tongan_label = gtk::Label::new(Some("Tongan"));
    grid.attach(&tongan_label, 0, 0, 1, 1);
    let tongan_entry = gtk::Entry::new();
    grid.attach(&tongan_entry, 1, 0, 1, 1);
    let english_label = gtk::Label::new(Some("English"));
    grid.attach(&english_label, 0, 1, 1, 1);
    let english_entry = gtk::Entry::new();
    grid.attach(&english_entry, 1, 1, 1, 1);
    let german_label = gtk::Label::new(Some("German"));
    grid.attach(&german_label, 0, 2, 1, 1);
    let german_entry = gtk::Entry::new();
    grid.attach(&german_entry, 1, 2, 1, 1);
    content.pack_start(&grid, true, true, 10);

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    content.pack_end(&separator, true, true, 10);

    dialog.connect_response(glib::clone!(@weak parent => move |_, response_type| {
        if response_type == gtk::ResponseType::Accept {
            let conn = match get_connection() {
                Ok(conn) => conn,
                Err(err) => {
                    show_error(&parent, &err.to_string());
                    return;
                }
            };
            let tongan = tongan_entry.get_buffer().get_text();
            let english = english_entry.get_buffer().get_text();
            let german = german_entry.get_buffer().get_text();
            match Card::add(&conn, 1, &tongan, &english, &german) {
                Ok(card_id) => replace_card(&parent, card_id),
                Err(err) => show_error(&parent, &err.to_string())
            }
        }
    }));

    dialog.show_all();
    dialog.run();
    dialog.close();
}

/// Show an error dialog
pub fn show_error(parent: &gtk::ApplicationWindow, message: &str) {
    let dialog = gtk::MessageDialog::new(
       Some(parent),
       gtk::DialogFlags::DESTROY_WITH_PARENT,
       gtk::MessageType::Error,
       gtk::ButtonsType::Close,
       message,
    );
    dialog.set_modal(true);
    dialog.run();
    dialog.close();
}