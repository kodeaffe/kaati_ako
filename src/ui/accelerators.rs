//! Handle application accelerators

use gtk::GtkApplicationExt;


/// Add accelerators for the application
///
/// Currently it handles 'F1', 'n' and 'Ctrl-Q'
pub fn add_accelerators(application: &gtk::Application) {
    application.set_accels_for_action("app.about", &["F1"]);
    application.set_accels_for_action("app.next_card", &["n"]);
    // `Primary` is a platform-agnostic accelerator modifier.
    // On Windows and Linux, `Primary` maps to the `Ctrl` key,
    // and on macOS it maps to the `command` key.
    application.set_accels_for_action("app.add_card", &["<Primary>A"]);
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
}
