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

mod database;
mod models;
mod ui;

use ui::UI;


/// Default path to database file
const DEFAULT_DB_PATH: &str = "kaati_ako.sqlite";

/// The version of application (ideally, this would be taken from `Cargo.toml`)
const VERSION: &str = "0.1.0";


/// Build the application and run it
fn main() {
    let application = gtk::Application::new(
        Some("com.github.kodeaffe.kaati_ako"), Default::default()).unwrap();
    application.connect_activate(|app| {
        UI::build(app);
    });
    application.run(&std::env::args().collect::<Vec<_>>());
}