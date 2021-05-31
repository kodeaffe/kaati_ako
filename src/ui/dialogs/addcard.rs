//! Module for the dialog to add a new flash card

use glib::Cast;
use gtk::{BoxExt, ContainerExt, DialogExt, EntryExt, GridExt, GtkWindowExt, WidgetExt};

use crate::database::{DatabaseError, get_connection};
use crate::models::Model;
use crate::models::card::Card;
use crate::models::category::Category;
use crate::models::language::Language;
use crate::models::translation::Translation;
use crate::ui::widgets::card::Card as CardWidget;
use super::error::Error;


/// A dialog to add a flash card
pub struct AddCard;

/// Implementation of the dialog to add a flash card
impl AddCard {
    /// Build the language grid
    fn build_grid(languages: &Vec<Language>) -> Result<gtk::Grid, DatabaseError> {
        let grid = gtk::Grid::new();
        grid.set_column_spacing(5);
        grid.set_row_spacing(10);
        for language in languages {
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
        Ok(grid)
    }

    /// Build the dialog
    fn build(parent: &gtk::ApplicationWindow) -> Result<gtk::Dialog, DatabaseError> {
        let dialog = gtk::Dialog::with_buttons(
            Some("Add Card"),
            Some(parent),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            &[
                ("_Ok", gtk::ResponseType::Accept),
                ("_Cancel", gtk::ResponseType::Cancel),
            ],
        );
        let conn = get_connection()?;
        let languages = Language::load_all(&conn)?;
        let content = dialog.get_content_area();
        let text = gtk::Label::new(Some("Category: default"));
        content.pack_start(&text, true, true, 20);
        let grid = AddCard::build_grid(&languages)?;
        content.pack_start(&grid, true, true, 10);
        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        content.pack_end(&separator, true, true, 10);
        dialog.connect_response(glib::clone!(@weak parent => move |_, response_type| {
            if response_type == gtk::ResponseType::Accept {
                AddCard::response_accept(&parent, &conn, &languages, &grid);
            }
        }));
        Ok(dialog)
    }

    /// When the dialog is accepted, respond by saving the provided data into a new card and replace
    /// the currently shown CardWidget
    fn response_accept(
        parent: &gtk::ApplicationWindow,
        conn: &sqlite::Connection,
        languages: &Vec<Language>,
        grid: &gtk::Grid,
    ) {
        // TODO: Get category id from user
        let category = match Category::load(&conn, 1) {
            Ok(category) => category,
            Err(err) => {
                Error::show(&parent, &err.to_string());
                return;
            }
        };
        let mut card = Card::new(category.id);
        match card.save(&conn) {
            Ok(id) => card.id = id,
            Err(err) => {
                Error::show(&parent, &err.to_string());
                return;
            },
        }
        for language in languages {
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
                card.id,
                language.id,
                text.clone(),
                description.clone(),
            );
            match translation.save(&conn) {
                Ok(id) => translation.id = id,
                Err(err) => {
                    Error::show(&parent, &err.to_string());
                    return;
                },
            }
        }
        CardWidget::replace(&parent, card.id);
    }

    /// Show the dialog
    pub fn show(parent: &gtk::ApplicationWindow) {
        let dialog = match AddCard::build(parent) {
            Ok(dialog) => dialog,
            Err(err) => {
                Error::show(&parent, &err.to_string());
                return;
            },
        };
        dialog.show_all();
        dialog.run();
        dialog.close();
    }
}