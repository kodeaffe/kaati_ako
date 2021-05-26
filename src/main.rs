//! Kaati Ako
//!
//! This is a little application to learn another language by flash cards. 'kaati ako' means 'flash
//! card' in Tongan.
//! You simply run it without any parameters.
//!
//! # Panics
//! It currently panics on startup when the prepared database file can not be found in the current
//! directory.

use std::cell::RefCell;

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use sqlite;

mod ui;
mod util;

use ui::build;
//use util::database::{Card, Translation, connect_database};


/// Default path to database file
const DEFAULT_DB_PATH: &str = "kaati_ako.sqlite";

/// The version of application (ideally, this would be taken from `Cargo.toml`)
const VERSION: &str = "0.1.0";

thread_local!(
    /// Thread-local variable to hold the database connection
    pub static DB_CONNECTION: RefCell<Option<sqlite::Connection>> = RefCell::new(None)
);


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
    application.add_main_option(
        "db_path",
        glib::Char::new('d').unwrap(),
        glib::OptionFlags::OPTIONAL_ARG,
        glib::OptionArg::String,
        "Use a custom database file",
        Some("FILENAME"),
    );
    application.connect_handle_local_options(|_, options| {
        // Read path to DB from command-line
        let db_path = options.lookup_value("db_path", None);
        let db_path = match db_path {
            Some(path) => {
                let path = path.to_string();
                // Is this a bug in gtk-rs? Need to strip extra ' from first and last index
                let path = path[1..path.len() - 1].to_string();
                if std::path::Path::new(&path).exists() {
                    path
                } else {
                    eprintln!("Database file '{}' does not exist.", path);
                    std::process::exit(1);
                }
            },
            None => DEFAULT_DB_PATH.to_string(),
        };
        // Open a connection to database and put it into thread-local storage
        DB_CONNECTION.with(|cell| {
            *cell.borrow_mut() = match sqlite::open(&db_path) {
                Ok(conn) => Some(conn),
                Err(err) => {
                    eprintln!("{}", err.to_string());
                    std::process::exit(2);
                },
            }
        });
        -1 // Application will continue running
    });
    application.connect_activate(|app| {
        build(app);
    });
    application.run(&std::env::args().collect::<Vec<_>>());
}