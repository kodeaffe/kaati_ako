//! Handle the database
//!
//! # Example
//!
//! ```rust
//! use database::{get_connection, initialise};
//! let conn = get_connection()?;
//! initialise(conn);
//! ```

use std::env;
use std::error::Error;
use std::fmt;
use std::path;

use sqlite;

use crate::DEFAULT_DB_PATH;


/// An custom error which can occur during access to the database
#[derive(Debug)]
pub enum DatabaseError {
    /// A file could not be found, the filename should be in the string
    FileNotFound(String),
    /// Item has not been found
    NotFound,
    /// An error within SQLite occurred, the error message should be in the string
    SQLiteError(String),
    /// The value returned by the database is not the expected integer
    ValueNotInteger,
    /// The value returned by the database is not the expected string
    ValueNotString,

}

/// The implementation of the Error trait is empty
impl Error for DatabaseError {}


/// Implement the From trait to convert a sqlite::Error to a DatabaseError
impl From<sqlite::Error> for DatabaseError {
    fn from(err: sqlite::Error) -> Self {
        DatabaseError::SQLiteError(err.to_string())
    }
}


/// Implement the Display trait to show a DatabaseError
impl fmt::Display for DatabaseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let prefix = "DatabaseError";
      match self {
          DatabaseError::FileNotFound(db_path) =>
              write!(f, "{}: File not found: {}!", prefix, db_path),
          DatabaseError::NotFound => write!(f, "{}: Item has not been found!", prefix),
          DatabaseError::SQLiteError(msg) =>
              write!(f, "{}: SQLite error: {}!", prefix, msg),
          DatabaseError::ValueNotInteger => write!(f, "{}: Value not an integer!", prefix),
          DatabaseError::ValueNotString => write!(f, "{}: Value not a string!", prefix),
      }
  }
}


/// Get a database connection using a DB path from the environment
pub fn get_connection() -> Result<sqlite::Connection, DatabaseError> {
    let db_path = env::var("DB_PATH").unwrap_or(DEFAULT_DB_PATH.to_string());
    if !path::Path::new(&db_path).exists() {
        return Err(DatabaseError::FileNotFound(db_path));
    }
    let conn = sqlite::open(db_path)?;
    Ok(conn)
}


/// Create a new database from scratch, including some fixture data
#[allow(dead_code)]
pub fn initialise(conn: &sqlite::Connection) -> Result<(), DatabaseError> {
    init_schema(conn)?;
    init_fixtures(conn)?;
    Ok(())
}


/// Initialise the database fixtures
fn init_fixtures(conn: &sqlite::Connection) -> Result<(), DatabaseError> {
    let result = conn.execute("
        INSERT INTO language (code, name) VALUES ('to', 'Tongan');
        INSERT INTO language (code, name) VALUES ('en', 'English');
        INSERT INTO language (code, name) VALUES ('de', 'German');
        INSERT INTO category (name) VALUES ('default');
        INSERT INTO card (category_id) VALUES (1);
        INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 1, 'kaati', '');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 2, 'card', 'A card as in flash card or birthday card');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 3, 'Karte', 'Eine Karte wie in Karteikarte oder Geburtstagskarte');
        INSERT INTO card (category_id) VALUES (1);
        INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 1, 'ako', '');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 2, 'learn', 'Learn a language');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 3, 'lernen', 'Eine Sprache lernen');
        INSERT INTO card (category_id) VALUES (1);
        INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 1, 'lea faka', '');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 2, 'language', 'Learn a language');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 3, 'Sprache', 'Eine Sprache lernen');
        ")?;
    Ok(result)
}


/// Initialise the database schema
fn init_schema(conn: &sqlite::Connection) -> Result<(), DatabaseError> {
    let result = conn.execute("
        DROP TABLE IF EXISTS category;
        CREATE TABLE category (
            id INTEGER NOT NULL PRIMARY KEY,
            name TEXT
        );
        DROP TABLE IF EXISTS card;
        CREATE TABLE card (
            id INTEGER NOT NULL PRIMARY KEY,
            category_id INTEGER,
            FOREIGN KEY (category_id) REFERENCES category (id)
        );
        DROP TABLE IF EXISTS language;
        CREATE TABLE language (
            id INTEGER NOT NULL PRIMARY KEY,
            code TEXT,
            name TEXT
        );
        DROP TABLE IF EXISTS translation;
        CREATE TABLE translation (
            id INTEGER NOT NULL PRIMARY KEY,
            card_id INTEGER,
            language_id INTEGER,
            text TEXT,
            description TEXT,
            FOREIGN KEY (card_id) REFERENCES card (id),
            FOREIGN KEY (language_id) REFERENCES language (id)
        );
    ")?;
    Ok(result)
}


/// Get the identifier of the last inserted item in the given table
pub fn last_insert_id(conn: &sqlite::Connection, table_name: &str) -> Result<i64, DatabaseError> {
    // Cannot prepare `SELECT last_insert_rowid() FROM ?` ... Bug?
    let statement = format!("SELECT last_insert_rowid() FROM {}", table_name);
    let mut cursor = conn.prepare(&statement)?.cursor();
    for row in cursor.next()? {
        match row[0].as_integer() {
            Some(id) => { return Ok(id); },
            None => { return Err(DatabaseError::ValueNotInteger); },
        }
    }
    Ok(0)
}