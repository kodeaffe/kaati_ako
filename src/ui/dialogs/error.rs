//! Module for the error dialog

use gtk::{DialogExt, GtkWindowExt};


/// A dialog to show an error window
pub struct Error;

/// Implementation of the dialog to show an error
impl Error {

    /// Show the dialog
    ///
    /// # Arguments
    ///
    /// * `parent` - The GTK application window
    /// * `message` - String slice holding the error message
    pub fn show(parent: &gtk::ApplicationWindow, message: &str) {
        let dialog = gtk::MessageDialog::new(
           Some(parent),
           gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
           gtk::MessageType::Error,
           gtk::ButtonsType::Close,
           message,
        );
        dialog.run();
        dialog.close();
    }
}