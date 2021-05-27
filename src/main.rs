//! Kaati Ako
//!
//! This is a little application to learn another language by flash cards. 'kaati ako' means 'flash
//! card' in Tongan.
//! You can simply run it without any parameters to use the default database:
//!
//! ```sh
//! kaati_ako
//! ```
//!
//! Or you can specify the database file by an environment variable:
//!
//! ```sh
//! DB_PATH=db.sqlite kaati_ako
//! ```


use gio::prelude::{ApplicationExt, ApplicationExtManual};

mod ui;
mod util;

use ui::build;
//use util::database::{Card, Translation, connect_database};


/// Default path to database file
const DEFAULT_DB_PATH: &str = "kaati_ako.sqlite";

/// The version of application (ideally, this would be taken from `Cargo.toml`)
const VERSION: &str = "0.1.0";


/// Build the application and run it
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
    application.run(&std::env::args().collect::<Vec<_>>());
}