//! Handle the database

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
          DatabaseError::SQLiteError(msg) =>
              write!(f, "{}: SQLite error: {}!", prefix, msg),
          DatabaseError::ValueNotInteger => write!(f, "{}: Value not an integer!", prefix),
          DatabaseError::ValueNotString =>
              write!(f, "{}: Value not a string!", prefix),
      }
  }
}


/// A flash card category
#[derive(Debug)]
pub struct Category {
    /// Identifier of the category
    pub id: i64,
    /// Name of the category
    pub name: String,
}

/// The language of a flash card translation
#[derive(Debug)]
pub struct Language {
    /// Identifier of the language
    pub id: i64,
    /// Code of the language as in [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    pub code: String,
    /// Name of the language
    pub name: String,
}

/// A flash card's translation
#[derive(Debug)]
pub struct Translation {
    /// Identifier of the translation
    pub id: i64,
    /// Language the translation is made in
    pub language: Language,
    /// The value of the translation
    pub text: String,
    /// An optional description with examples or further explanations
    pub description: String,
}

#[allow(dead_code)]
impl Translation {
    /// Insert a translation in a given language for a given card
    pub fn add(
        conn: &sqlite::Connection,
        card_id: i64,
        language_id: i64,
        text: &str,
        description: &str,
    ) -> Result<i64, DatabaseError> {
        let statement = "
            INSERT INTO translation (card_id, language_id, text, description) VALUES (?, ?, ?, ?)
        ";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[
            sqlite::Value::Integer(card_id),
            sqlite::Value::Integer(language_id),
            sqlite::Value::String(text.to_string()),
            sqlite::Value::String(description.to_string()),
        ])?;
        cursor.next()?;
        last_insert_id(conn, "translation")
    }

    /// Select all translations for a given card
    pub fn get_all(
        conn: &sqlite::Connection,
        card_id: i64,
    ) -> Result<Vec<Translation>, DatabaseError> {
        let statement = "
            SELECT translation.id, language.id, language.code, language.name, text, description
            FROM translation
            LEFT JOIN language ON translation.language_id = language.id
            WHERE card_id = ?
        ";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(card_id)])?;
        let mut translations = Vec::new();
        while let Some(row) = cursor.next()? {
            let language_id = match row[1].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            let language_code = match row[2].as_string() {
                Some(code) => code.to_string(),
                None => { return Err(DatabaseError::ValueNotString); },
            };
            let language_name = match row[3].as_string() {
                Some(name) => name.to_string(),
                None => { return Err(DatabaseError::ValueNotString); },
            };
            let language = Language { id: language_id, code: language_code, name: language_name };
            let id = match row[1].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            let text = match row[4].as_string() {
                Some(text) => text.to_string(),
                None => { return Err(DatabaseError::ValueNotString); },
            };
            let description = match row[5].as_string() {
                Some(description) => description.to_string(),
                None => { return Err(DatabaseError::ValueNotString); },
            };
            translations.push(Translation { id, language, text, description });
        }
        Ok(translations)
    }
}

/// A flash card
#[derive(Debug)]
pub struct Card {
    /// Identifier of the card
    pub id: i64,
    /// Category of the card
    pub category: Category,
    /// Translations for the card
    pub translations: Vec<Translation>,
}

#[allow(dead_code)]
impl Card {
    /// Insert a flash card for given category
    pub fn add(conn: &sqlite::Connection, category_id: i64) -> Result<i64, DatabaseError> {
        let statement = "INSERT INTO card (category_id) VALUES (?)";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(category_id)])?;
        cursor.next()?;
        last_insert_id(conn, "card")
    }

    /// Select all flash cards with translations
    #[allow(dead_code)]
    pub fn get_all(conn: &sqlite::Connection) -> Result<Vec<Card>, DatabaseError> {
        let statement = "
            SELECT card.id, category.id, category.name
            FROM card
            LEFT JOIN category ON card.category_id = category.id
            ORDER BY card.id
        ";
        let mut cursor = conn.prepare(statement)?.cursor();
        let mut cards = Vec::new();
        while let Some(row) = cursor.next()? {
            let card_id = match row[0].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            let category_id = match row[0].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            let category_name = match row[0].as_string() {
                Some(name) => name.to_string(),
                None => { return Err(DatabaseError::ValueNotString); },
            };
            let card = Card {
                id: card_id,
                category: Category { id: category_id, name: category_name },
                translations: Translation::get_all(conn, card_id)?,
            };
            cards.push(card);
        }
        Ok(cards)
    }

    /// Instantiate an empty card
    pub fn get_empty() -> Card {
        Card {
            id: 0,
            category: Category { id: 0, name: "".to_string() },
            translations: Vec::new(),
        }
    }

    /// Select a random flash card with translations
    pub fn get_random(conn: &sqlite::Connection) -> Result<Card, DatabaseError> {
        let statement = "
            SELECT card.id, category.id, category.name
            FROM card
            LEFT JOIN category ON card.category_id = category.id
            ORDER BY RANDOM()
            LIMIT 1
        ";
        let mut cursor = conn.prepare(statement)?.cursor();
        while let Some(row) = cursor.next()? {
            let card_id = match row[0].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            let category_id = match row[1].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            let category_name = match row[2].as_string() {
                Some(name) => name,
                None => { return Err(DatabaseError::ValueNotString); },
            };
            return Ok(Card {
                id: card_id,
                category: Category { id: category_id, name: category_name.to_string() },
                translations: Translation::get_all(&conn, card_id)?,
            });
        }
        Ok(Card::get_empty())
    }
}

/// Create a new database from scratch, including some fixture data
#[allow(dead_code)]
pub fn create_database(conn: &sqlite::Connection) -> Result<(), DatabaseError> {
    let result = conn.execute("
        DROP TABLE IF EXISTS category;
        CREATE TABLE category (
            id INTEGER NOT NULL PRIMARY KEY,
            name TEXT
        );
        INSERT INTO category (name) VALUES ('default');

        DROP TABLE IF EXISTS card;
        CREATE TABLE card (
            id INTEGER NOT NULL PRIMARY KEY,
            category_id INTEGER,
            FOREIGN KEY (category_id) REFERENCES category (id)
        );
        INSERT INTO card (category_id) VALUES (1);
        INSERT INTO card (category_id) VALUES (1);
        INSERT INTO card (category_id) VALUES (1);

        DROP TABLE IF EXISTS language;
        CREATE TABLE language (
            id INTEGER NOT NULL PRIMARY KEY,
            code TEXT,
            name TEXT
        );
        INSERT INTO language (code, name) VALUES ('to', 'Tongan');
        INSERT INTO language (code, name) VALUES ('en', 'English');
        INSERT INTO language (code, name) VALUES ('de', 'German');

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
        INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 1, 'kaati', '');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 2, 'card', 'A card as in flash card or birthday card');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (1, 3, 'Karte', 'Eine Karte wie in Karteikarte oder Geburtstagskarte');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 1, 'ako', '');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 2, 'learn', 'Learn a language');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (2, 3, 'lernen', 'Eine Sprache lernen');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 1, 'lea faka', '');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 2, 'language', 'Learn a language');
        INSERT INTO translation (card_id, language_id, text, description) VALUES (3, 3, 'Sprache', 'Eine Sprache lernen');
        ")?;
    //println!("cards: {:?}", Card::get_all(&conn);
    Ok(result)
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


/// Get the identifier of the last inserted item in the given table
#[allow(dead_code)]
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