//! Module for the about dialog

use std::fs;

use gtk::{AboutDialogExt, GtkWindowExt, WidgetExt};

use crate::VERSION;
use super::error::Error;


/// Dialog to show an about window
pub struct About;

/// Implementation of the about dialog
impl About {
    /// Show the dialog
    pub fn show(parent: &gtk::ApplicationWindow) {
        let dialog = gtk::AboutDialog::new();
        dialog.set_authors(&["kodeaffe <lahi+kodeaffe@posteo.de>"]);
        let licence_path = "LICENSE";
        let licence = match fs::read_to_string(licence_path) {
            Ok(licence) => licence,
            Err(err) => {
                Error::show(parent, &format!("{}: {}", err, licence_path));
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
}