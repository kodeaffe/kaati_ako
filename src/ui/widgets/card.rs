//! Module for the flash card widget

use glib::{Cast, ObjectExt};
use gtk::{BoxExt, ContainerExt, LabelExt, Notebook, WidgetExt};
use gtk::prelude::NotebookExtManual;

use crate::database::{get_connection, DatabaseError};
use crate::models::Model;
use crate::models::card::Card as CardModel;
use crate::models::category::Category;
use crate::models::language::Language;
use crate::models::translation::Translation;
use crate::ui::{WIDGET_NAME_CARD, WIDGET_NAME_CONTENT};
use crate::ui::dialogs::error::Error;


/// A widget for a flash card
pub struct Card;

/// Implementation of the flash card widget
impl Card {
    /// Build a card's notebook page for the given translation
    fn build_page(
        conn: &sqlite::Connection,
        category: &Category,
        translation: &Translation,
    ) -> Result<(gtk::Box, gtk::Label), DatabaseError> {
        let padding = 10;
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

        let language = Language::load(&conn, translation.language_id)?;
        let label = gtk::Label::new(Some(&language.name));
        Ok((page, label))
    }

    /// Get a card with given id from database, including category and translations
    fn get_card(conn: &sqlite::Connection, card_id: i64) -> Result<CardModel, DatabaseError> {
        let id = if card_id == 0 { CardModel::random_id(&conn)? } else { card_id };
        let mut card = CardModel::load(conn, id)?;
        card.category = Category::load(conn, card.category_id)?;
        card.translations = Translation::load_for_card(conn, card.id)?;
        Ok(card)
    }

    /// Build a flash card as Notebook widget, uses a random card if given card_id has value 0
    pub fn build(window: &gtk::ApplicationWindow, card_id: i64) -> gtk::Notebook {
        let notebook = gtk::Notebook::new();
        notebook.set_widget_name(WIDGET_NAME_CARD);
        notebook.grab_focus();

        let conn = match get_connection() {
            Ok(conn) => conn,
            Err(err) => {
                Error::show(window, &err.to_string());
                return notebook;
            }
        };
        let card = match Card::get_card(&conn, card_id) {
            Ok(card) => card,
            Err(err) => {
                Error::show(window, &err.to_string());
                return notebook;
            }
        };
        for translation in card.translations {
            match Card::build_page(&conn, &card.category, &translation) {
                Ok((page, label)) => {
                    notebook.append_page(&page, Some(&label));
                }
                Err(err) => {
                    Error::show(window, &err.to_string());
                    return notebook;
                }
            }
        }
        unsafe {
            notebook.set_data("card_id", card.id);
        }
        notebook
    }

    /// Find the currently shown card widget
    pub fn find(window: &gtk::ApplicationWindow) -> Option<Notebook> {
        // TODO: Is there a better way to find the box and card?
        for widget in window.get_children() {
            if widget.get_widget_name() == WIDGET_NAME_CONTENT {
                match widget.downcast::<gtk::Box>() {
                    Ok(vbox) => {
                        for child in vbox.get_children() {
                            if child.get_widget_name() == WIDGET_NAME_CARD {
                                match child.downcast::<gtk::Notebook>() {
                                    Ok(card) => { return Some(card); },
                                    _ => {},
                                }
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
        None
    }

    /// Replace the shown flash card by the card with given id
    pub fn replace(window: &gtk::ApplicationWindow, card_id: i64) {
        match Card::find(window) {
            Some(card) => {
                match card.get_parent() {
                    Some(parent) => {
                        match parent.downcast::<gtk::Box>() {
                            Ok(vbox) => {
                                vbox.remove(&card);
                                let card = Card::build(window, card_id);
                                vbox.pack_start(&card, true, true, 10);
                                vbox.show_all();
                                // Focus must be grabbed after being shown
                                card.grab_focus();
                            }
                            _ => {},
                        }
                    },
                    None => {},
                }
            }
            None => {},
        }
    }
}