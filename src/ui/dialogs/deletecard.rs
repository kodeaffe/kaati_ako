//! Module for the dialog to delete a flash card

use gtk::{BoxExt, DialogExt, GtkWindowExt, WidgetExt};

use crate::database::get_connection;
use crate::models::Model;
use crate::models::card::Card;
use crate::ui::widgets::cardnotebook::CardNotebook;
use super::error::Error as ErrorDialog;


/// A dialog to delete a card
pub struct DeleteCard;

/// Implementation of the dialog to delete a card
impl DeleteCard {

    /// Show the dialog
    pub fn show(parent: &gtk::ApplicationWindow) {
        let dialog = gtk::Dialog::with_buttons(
            Some("Delete Card"),
            Some(parent),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            &[
                ("_Ok", gtk::ResponseType::Accept),
                ("_Cancel", gtk::ResponseType::Cancel),
            ],
        );
        let spacing = 10;
        let content = dialog.get_content_area();
        content.set_margin_start(spacing as i32);
        content.set_margin_end(spacing as i32);
        let label = gtk::Label::new(
            Some("Are you sure you want to delete the current card?"));
        content.pack_start(&label, true, true, spacing);
        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        content.pack_end(&separator, false, false, spacing);
        dialog.connect_response(glib::clone!(@weak parent => move |_, response_type| {
            if response_type == gtk::ResponseType::Accept {
                DeleteCard::response_accept(&parent);
            }
        }));
        dialog.show_all();
        dialog.run();
        dialog.close();
    }

    /// When the dialog is accepted, delete the card and replace it with a random card
    fn response_accept(parent: &gtk::ApplicationWindow) {
        let conn = match get_connection() {
            Ok(conn) => conn,
            Err(err) => {
                ErrorDialog::show(parent, &err.to_string());
                return;
            }
        };
        let card_id = match CardNotebook::get_card_id(parent) {
            Ok(id) => id,
            Err(err) => {
                ErrorDialog::show(parent, &err.to_string());
                return;
            }
        };
        match Card::delete(&conn, card_id) {
            Err(err) => {
                ErrorDialog::show(parent, &err.to_string());
                return;
            },
            _ => {},
        }
        match Card::random_id(&conn) {
            Ok(id) => CardNotebook::replace(&parent, id),
            Err(err) => ErrorDialog::show(parent, &err.to_string()),
        }
    }
}