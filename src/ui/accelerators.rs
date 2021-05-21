use gtk::GtkApplicationExt;


pub fn add_accelerators(application: &gtk::Application) {
    application.set_accels_for_action("app.about", &["F1"]);
    application.set_accels_for_action("app.next_card", &["n"]);
    // `Primary` is a platform-agnostic accelerator modifier.
    // On Windows and Linux, `Primary` maps to the `Ctrl` key,
    // and on macOS it maps to the `command` key.
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
}
