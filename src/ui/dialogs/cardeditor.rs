//! Module for the dialog to add a new flash card

use std::error::Error;

use glib::Cast;
use gtk::{
    BoxExt,
    ComboBoxTextExt,
    ContainerExt,
    DialogExt,
    EntryExt,
    GridExt,
    GtkWindowExt,
    WidgetExt,
    prelude::ComboBoxExtManual,
};

use crate::database::{DatabaseError, get_connection};
use crate::models::Model;
use crate::models::card::Card;
use crate::models::category::Category;
use crate::models::language::Language;
use crate::models::translation::Translation;
use crate::ui::widgets::cardnotebook::CardNotebook;
use super::error::Error as ErrorDialog;


/// A dialog to add a flash card
pub struct CardEditor;

/// Implementation of the dialog to add a flash card
impl CardEditor {
    /// Build category widget
    ///
    /// Set `selected` to 0 to have the `default` category selected
    fn build_category(
        conn: &sqlite::Connection,
        selected: i64,
    ) -> Result<gtk::ComboBoxText, DatabaseError> {
        let combo = gtk::ComboBoxText::new();
        let categories = Category::load_all(&conn)?;
        let mut active_idx = 0;
        for (idx, category) in categories.iter().enumerate() {
            combo.append_text(&category.name);
            if category.id == selected {
                active_idx = idx;
            } else if selected == 0 && category.name == "default" {
                active_idx = idx;
            }
        }
        combo.set_active(Some(active_idx as u32));
        Ok(combo)
    }

    /// Build the translation grid
    fn build_translations(
        conn: &sqlite::Connection,
        card_id: i64,
        languages: &Vec<Language>,
    ) -> Result<gtk::Grid, DatabaseError> {
        let grid = gtk::Grid::new();
        grid.set_column_spacing(5);
        grid.set_row_spacing(10);
        for language in languages {
            let translation = Translation::load_for_card_language(
                &conn, card_id, language.id)?;
            let label = gtk::Label::new(Some(&language.name));
            label.set_halign(gtk::Align::Start);
            // Map language id to top: 1 -> 0, 1 ; 2 -> 2, 3 ; 3 -> 4, 5
            let mut top = language.id as i32 * 2 - 2 ;
            grid.attach(&label, 0, top, 1, 1);
            let text = gtk::Entry::new();
            text.set_text(&translation.text);
            text.set_placeholder_text(Some("Add text ..."));
            text.set_widget_name(&format!("text_{}", language.id));
            top += 1;
            grid.attach(&text, 0, top, 1, 1);
            let description = gtk::Entry::new();
            description.set_text(&translation.description);
            description.set_placeholder_text(Some("Add description ..."));
            description.set_widget_name(&format!("description_{}", language.id));
            grid.attach(&description, 1, top, 1, 1);
        }
        Ok(grid)
    }

    /// Build the dialog
    fn build(parent: &gtk::ApplicationWindow, card_id: i64) -> Result<gtk::Dialog, DatabaseError> {
        let dialog = gtk::Dialog::with_buttons(
            if card_id > 0 { Some("Edit Card") } else { Some("Add Card") },
            Some(parent),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            &[
                ("_Ok", gtk::ResponseType::Accept),
                ("_Cancel", gtk::ResponseType::Cancel),
            ],
        );
        let conn = get_connection()?;
        let card = CardEditor::get_card(&conn, card_id)?;
        let spacing = 10;
        let languages = Language::load_all(&conn)?;

        let content = dialog.get_content_area();
        content.set_margin_start(spacing as i32);
        content.set_margin_end(spacing as i32);

        let label = gtk::Label::new(Some("Category"));
        label.set_halign(gtk::Align::Start);
        content.pack_start(&label, false, false, spacing);

        let combo = CardEditor::build_category(&conn, card.category_id)?;
        content.pack_start(&combo, false, false, spacing);

        let grid = CardEditor::build_translations(&conn, card.id, &languages)?;
        content.pack_start(&grid, false, false, spacing);

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        content.pack_end(&separator, false, false, spacing);

        dialog.connect_response(glib::clone!(@weak parent => move |_, response_type| {
            if response_type == gtk::ResponseType::Accept {
                CardEditor::response_accept(&parent, &conn, card.id, &languages, &combo, &grid);
            }
        }));
        Ok(dialog)
    }

    /// Handle the category when the dialog has been accepted
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    /// * `category_widget` - Widget which holds the category
    /// * `card_id` - Identifier of the card which was accepted
    fn accept_category(
        conn: &sqlite::Connection,
        category_widget: &gtk::ComboBoxText,
        card_id: i64,
    ) -> Result<Card, Box<dyn Error>> {
        let mut card = CardEditor::get_card(&conn, card_id)?;
        let category_id = match category_widget.get_active_text() {
            Some(name)  => {
                let category = Category::load_by_name(&conn, name.to_string())?;
                category.id
            },
            None => Err("No category selected!")?,
        };
        if category_id != card.category_id {
            card.category_id = category_id;
            card.save(&conn)?;
        }
        Ok(card)
    }

    /// Handle a single translation when the dialog has been accepted for a given language
    ///
    /// # Arguments
    ///
    /// * `conn` -
    /// * `translations_widget` - Widget which holds all translation texts & descriptions
    /// * `card` - Card which has been accepted
    /// * `language` - Language to process
    fn accept_translation(
        conn: &sqlite::Connection,
        card_id: i64,
        translations_widget: &gtk::Grid,
        language: &Language,
    ) -> Result<Translation, DatabaseError> {
        let mut text = String::new();
        let name_text = format!("text_{}", language.id);
        let mut description = String::new();
        let name_description = format!("description_{}", language.id);
        // FIXME: Better way to get the widget?
        for child in translations_widget.get_children() {
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
        let mut translation = Translation::load_for_card_language(
            &conn, card_id, language.id)?;
        translation.text = text.clone();
        translation.description = description.clone();
        translation.save(&conn)?;
        Ok(translation)
    }

    /// Get Card by given card id
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    /// * `card_id` - Identifier of the card to get. If 0, an empty card will be retrieved.
    fn get_card(conn: &sqlite::Connection, card_id: i64) -> Result<Card, DatabaseError> {
        let card = if card_id == 0 {
            Card::from_empty()
        } else {
            Card::load(&conn, card_id)?
        };
        Ok(card)
    }

    /// When the dialog is accepted, respond by saving the provided data into a new card and replace
    /// the currently shown card
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent widget of the dialog aka application window
    /// * `conn`- Connection to the database
    /// * `languages` - A vector with all supported languages
    /// * `category_widget` - The widget which holds the accepted category
    /// * `translations_widget` - The widget which holds the accepted translations
    /// * `card_id` - Identifier of the card to handle, cannot be a `Card` because of:
    ///   ```cannot borrow `card` as mutable, as it is a captured variable in a `Fn` closure```
    fn response_accept(
        parent: &gtk::ApplicationWindow,
        conn: &sqlite::Connection,
        card_id: i64,
        languages: &Vec<Language>,
        category_widget: &gtk::ComboBoxText,
        translations_widget: &gtk::Grid,
    ) {
        let card = match CardEditor::accept_category(&conn, &category_widget, card_id) {
            Ok(card) => card,
            Err(err) => {
                ErrorDialog::show(&parent, &err.to_string());
                return;
            }
        };
        for language in languages {
            match CardEditor::accept_translation(&conn, card.id, &translations_widget, &language) {
                Err(err) => {
                    ErrorDialog::show(&parent, &err.to_string());
                    return;
                },
                _ => {},
            }
        }
        CardNotebook::replace(&parent, card.id);
    }

    /// Show the dialog for card given by id
    fn show(parent: &gtk::ApplicationWindow, card_id: i64) {
        let dialog = match CardEditor::build(parent, card_id) {
            Ok(dialog) => dialog,
            Err(err) => {
                ErrorDialog::show(&parent, &err.to_string());
                return;
            },
        };
        dialog.show_all();
        dialog.run();
        dialog.close();
    }

    /// Show the dialog for adding
    pub fn show_add(parent: &gtk::ApplicationWindow) {
        CardEditor::show(parent, 0);
    }

    /// Show the dialog for editing
    pub fn show_edit(parent: &gtk::ApplicationWindow) {
        let card_id = match CardNotebook::get_card_id(&parent) {
            Ok(id) => id,
            Err(err) => {
                ErrorDialog::show(&parent, &err.to_string());
                return;
            }
        };
        CardEditor::show(parent, card_id);
    }
}