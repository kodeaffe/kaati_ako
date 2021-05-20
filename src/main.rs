use std::env::args;

use gio::prelude::*;
use glib;
use gtk::prelude::*;


fn add_accelerators(application: &gtk::Application) {
    // `Primary` is a platform-agnostic accelerator modifier.
    // On Windows and Linux, `Primary` maps to the `Ctrl` key,
    // and on macOS it maps to the `command` key.
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
}


fn add_actions(application: &gtk::Application, window: &gtk::ApplicationWindow) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));
    application.add_action(&quit);
}


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Kaati Ako");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);
    add_actions(application, &window);

    let button = gtk::Button::with_label("Quit");
    button.set_action_name(Some("app.quit"));
    window.add(&button);

    window.show_all();
}


fn main() {
    let application = gtk::Application::new(
            Some("com.github.kodeaffe.kaati_ako"), Default::default()).unwrap();
    application.connect_startup(|app| {
        add_accelerators(app);
    });
    application.connect_activate(build_ui);
    application.run(&args().collect::<Vec<_>>());
}