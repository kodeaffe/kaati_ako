//! Model Translation

use sqlite;

use super::language::Language;
use crate::database::{DatabaseError, last_insert_id};


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
    /// Instantiate an empty translation
    pub fn from_empty() -> Translation {
        Translation {
            id: 0,
            language: Language::from_empty(),
            text: "".to_string(),
            description: "".to_string(),
        }
    }

    /// Load all translations for a given card from the database
    pub fn load_for_card(
        conn: &sqlite::Connection,
        card_id: i64,
    ) -> Result<Vec<Translation>, DatabaseError> {
        let statement = "
            SELECT translation.id, language.id, language.code, language.name, text, description
            FROM translation
            LEFT JOIN language ON translation.language_id = language.id
            WHERE translation.card_id = ?
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

    /// Instantiate a new translation for given language, text and description
    pub fn new(language: Language, text: String, description: String) -> Translation {
        Translation { id: 0, language, text, description }
    }

    /// Save a translation for a given card to the database
    pub fn save(&mut self, conn: &sqlite::Connection, card_id: i64) -> Result<i64, DatabaseError> {
        let statement = "
            INSERT INTO translation (card_id, language_id, text, description) VALUES (?, ?, ?, ?)
        ";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[
            sqlite::Value::Integer(card_id),
            sqlite::Value::Integer(self.language.id),
            sqlite::Value::String(self.text.clone()),
            sqlite::Value::String(self.description.clone()),
        ])?;
        cursor.next()?;
        self.id = last_insert_id(conn, "translation")?;
        Ok(self.id)
    }
}