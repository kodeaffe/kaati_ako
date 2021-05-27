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

    /// Insert a translation in a given language for a given card
    pub fn insert(
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
}