use std::env::args;

use gio::prelude::{ApplicationExt, ApplicationExtManual};

mod ui;
mod util;

use ui::build;
//use util::database::{Card, Translation, connect_database};


const VERSION: &str = "0.1.0";


fn main() {
    /*
    let conn = connect_database();
    let card_id = Card::add(&conn, 1);
    Translation::add(&conn, card_id, 1, "lahi", "");
    Translation::add(&conn, card_id, 2, "large", "Large or big");
    Translation::add(&conn, card_id, 3, "gross", "Ein grosser Schrank");
    //create_database(&conn);
     */

    let application = gtk::Application::new(
        Some("com.github.kodeaffe.kaati_ako"), Default::default()).unwrap();
    application.connect_activate(|app| {
        build(app);
    });
    application.run(&args().collect::<Vec<_>>());
}