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

use crate::models::Model;
use crate::models::card::Card;
use crate::models::category::Category;
use crate::models::language::Language;
use crate::models::translation::Translation;
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
    let card = match Card::load(&conn, id) {
        Ok(card) => card,
        Err(err) => {
            show_error(window, &err.to_string());
            Card::from_empty()
        }
    };
    let category = match Category::load(&conn, card.category_id) {
        Ok(category) => category,
        Err(err) => {
            show_error(window, &err.to_string());
            return notebook;
        }
    };
    let translations = match Translation::load_for_card(&conn, card.id) {
        Ok(translations) => translations,
        Err(err) => {
            show_error(window, &err.to_string());
            return notebook;
        }
    };

    let padding = 10;
    for translation in translations {
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
        let category_label = gtk::Label::new(Some(""));
        category_label.set_markup(&format!("Category: <b>{}</b>", category.name));
        page_bottom.pack_end(&category_label, false, false, padding);
        page.pack_start(&page_bottom, false, false, padding);

        let language = match Language::load(&conn, translation.language_id) {
            Ok(language) => language,
            Err(err) => {
                show_error(window, &err.to_string());
                return notebook;
            }
        };
        let tab_label = gtk::Label::new(Some(&language.name));
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

    let conn = match get_connection() {
        Ok(conn) => conn,
        Err(err) => {
            show_error(&parent, &err.to_string());
            return;
        }
    };
    let languages = match Language::load_all(&conn) {
        Ok(languages) => languages,
        Err(err) => {
            show_error(parent, &err.to_string());
            return;
        }
    };
    for language in &languages {
        let label = gtk::Label::new(Some(&language.name));
        label.set_halign(gtk::Align::Start);
        // Map language id to top: 1 -> 0, 1 ; 2 -> 2, 3 ; 3 -> 4, 5
        let mut top = language.id as i32 * 2 - 2 ;
        grid.attach(&label, 0, top, 1, 1);
        let text = gtk::Entry::new();
        text.set_placeholder_text(Some("Add text ..."));
        text.set_widget_name(&format!("text_{}", language.id));
        top += 1;
        grid.attach(&text, 0, top, 1, 1);
        let description = gtk::Entry::new();
        description.set_placeholder_text(Some("Add description ..."));
        description.set_widget_name(&format!("description_{}", language.id));
        grid.attach(&description, 1, top, 1, 1);
    }
    content.pack_start(&grid, true, true, 10);

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    content.pack_end(&separator, true, true, 10);

    dialog.connect_response(glib::clone!(@weak parent => move |_, response_type| {
        if response_type == gtk::ResponseType::Accept {
            // TODO: Get category id from user
            let category = match Category::load(&conn, 1) {
                Ok(category) => category,
                Err(err) => {
                    show_error(&parent, &err.to_string());
                    return;
                }
            };
            let mut card = Card::new(category.id);
            match card.save(&conn) {
                Ok(id) => card.id = id,
                Err(err) => {
                    show_error(&parent, &err.to_string());
                    return;
                },
            }
            for language in &languages {
                let mut text = String::new();
                let name_text = format!("text_{}", language.id);
                let mut description = String::new();
                let name_description = format!("description_{}", language.id);
                // FIXME: Better way to get the widget?
                for child in grid.get_children() {
                    let widget_name = child.get_widget_name();
                    if widget_name == name_text {
                        text = match child.downcast::<gtk::Entry>() {
                            Ok(entry) => entry.get_buffer().get_text(),
                            _ => "".to_string()
                        };
                    } else if widget_name == name_description {
                        description = match child.downcast::<gtk::Entry>() {
                            Ok(entry) => entry.get_buffer().get_text(),
                            _ => "".to_string()
                        };
                    }
                }
                let mut translation = Translation::new(
                    card.id, language.id, text.clone(), description.clone());
                match translation.save(&conn) {
                    Ok(id) => translation.id = id,
                    Err(err) => {
                        show_error(&parent, &err.to_string());
                        return;
                    },
                }
            }
            replace_card(&parent, card.id);
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