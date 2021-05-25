//! Handle the database

use sqlite::{Connection, Value, open};


/// Path to the database file
const DB_PATH: &str = "kaati_ako.sqlite";

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
        conn: &Connection, card_id: i64, language_id: i64, text: &str, description: &str) -> i64 {
        let mut cursor = conn
            .prepare("
                INSERT INTO translation (card_id, language_id, text, description) VALUES (?, ?, ?, ?)
            ").unwrap().cursor();
        cursor.bind(&[
            Value::Integer(card_id),
            Value::Integer(language_id),
            Value::String(text.to_string()),
            Value::String(description.to_string()),
        ]).unwrap();
        cursor.next().unwrap();
        last_insert_id(conn, "translation")
    }

    /// Select all translations for a given card
    pub fn get_all(conn: &Connection, card_id: i64) -> Vec<Translation> {
        let mut cursor = conn
            .prepare("
                SELECT translation.id, language.id, language.code, language.name, text, description
                FROM translation
                LEFT JOIN language ON translation.language_id = language.id
                WHERE card_id = ?
            ")
            .unwrap()
            .cursor();
        cursor.bind(&[Value::Integer(card_id)]).unwrap();
        let mut translations = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            let language = Language {
                id: row[1].as_integer().unwrap(),
                code: row[2].as_string().unwrap().to_string(),
                name: row[3].as_string().unwrap().to_string(),
            };
            translations.push(Translation {
                id: row[0].as_integer().unwrap(),
                language,
                text: row[4].as_string().unwrap().to_string(),
                description: row[5].as_string().unwrap().to_string(),
            })
        }
        translations
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
    pub fn add(conn: &Connection, category_id: i64) -> i64 {
        let mut cursor = conn
            .prepare("INSERT INTO card (category_id) VALUES (?)").unwrap().cursor();
        cursor.bind(&[Value::Integer(category_id)]).unwrap();
        cursor.next().unwrap();
        last_insert_id(conn, "card")
    }

    /// Select all flash cards with translations
    #[allow(dead_code)]
    pub fn get_all(conn: &Connection) -> Vec<Card> {
        let mut cursor = conn
            .prepare("
                SELECT card.id, category.id, category.name
                FROM card
                LEFT JOIN category ON card.category_id = category.id
                ORDER BY card.id
            ")
            .unwrap()
            .cursor();
        let mut cards = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            let card_id = row[0].as_integer().unwrap();
            let card = Card {
                id: card_id,
                category: Category {
                    id: row[1].as_integer().unwrap(),
                    name: row[2].as_string().unwrap().to_string(),
                },
                translations: Translation::get_all(conn, card_id),
            };
            cards.push(card);
        }
        cards
    }

    /// Select a random flash card with translations
    pub fn get_random(conn: &Connection) -> Card {
        let mut cursor = conn
            .prepare("
                SELECT card.id, category.id, category.name
                FROM card
                LEFT JOIN category ON card.category_id = category.id
                ORDER BY RANDOM()
                LIMIT 1
            ")
            .unwrap()
            .cursor();
        while let Some(row) = cursor.next().unwrap() {
            let card_id = row[0].as_integer().unwrap();
            return Card {
                id: card_id,
                category: Category {
                    id: row[1].as_integer().unwrap(),
                    name: row[2].as_string().unwrap().to_string(),
                },
                translations: Translation::get_all(conn, card_id),
            };
        }
        Card { id: 0, category: Category {id: 0, name: "".to_string()}, translations: Vec::new()}
    }
}


/// Create a connection to the database and return a handler to use in future queries
pub fn connect_database() -> Connection {
    open(DB_PATH).unwrap()
}


/// Create a new database from scratch, including some fixture data
#[allow(dead_code)]
pub fn create_database(conn: &Connection) {
    conn
        .execute("
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
        ")
        .unwrap();
    //println!("cards: {:?}", Card::get_all(&conn);
}

/// Get the identifier of the last inserted item in the given table
#[allow(dead_code)]
pub fn last_insert_id(conn: &Connection, table_name: &str) -> i64 {
    // Cannot prepare `SELECT last_insert_rowid() FROM ?` ... Bug?
    let statement = format!("SELECT last_insert_rowid() FROM {}", table_name);
    let mut cursor = conn.prepare(&statement).unwrap().cursor();
    for row in cursor.next().unwrap() {
        return row[0].as_integer().unwrap();
    }
    0
}