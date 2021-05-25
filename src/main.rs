//! Kaati Ako
//!
//! This is a little application to learn another language by flash cards. 'kaati ako' means 'flash
//! card' in Tongan.
//! You simply run it without any parameters.
//!
//! # Panics
//! It currently panics on startup when the prepared database file can not be found in the current
//! directory.

use std::env::args;

use gio::prelude::{ApplicationExt, ApplicationExtManual};

mod ui;
mod util;

use ui::build;
//use util::database::{Card, Translation, connect_database};


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
    application.run(&args().collect::<Vec<_>>());
}