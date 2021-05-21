use std::env::args;

use gio::prelude::{ApplicationExt, ApplicationExtManual};

mod ui;
mod util;

use ui::build;


const VERSION: &str = "0.1.0";


fn main() {
    /*
    let conn = connect_database();
    create_database(&conn);
    */

    let application = gtk::Application::new(
        Some("com.github.kodeaffe.kaati_ako"), Default::default()).unwrap();
    application.connect_activate(|app| {
        build(app);
    });
    application.run(&args().collect::<Vec<_>>());
}