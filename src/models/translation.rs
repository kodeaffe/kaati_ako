//! Model Translation

use sqlite;

use crate::database::DatabaseError;
use super::Model;


/// A flash card's translation
#[derive(Debug)]
pub struct Translation {
    /// Identifier of the translation
    pub id: i64,
    /// Card the translation belongs to
    pub card_id: i64,
    /// Language the translation is made in
    pub language_id: i64,
    /// The value of the translation
    pub text: String,
    /// An optional description with examples or further explanations
    pub description: String,
}


impl Translation {
    /// Load all translations for a given card from the database
    pub fn load_for_card(
        conn: &sqlite::Connection,
        card_id: i64,
    ) -> Result<Vec<Translation>, DatabaseError> {
        let statement = format!(
            "SELECT id FROM {} WHERE card_id = ? ORDER BY language_id", Translation::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(card_id)])?;
        let mut translations = Vec::new();
        while let Some(row) = cursor.next()? {
            let id = match row[0].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            translations.push(Translation::load(conn, id)?);
        }
        Ok(translations)
    }

    /// Instantiate a new Translation for given card, language, text and description
    pub fn new(card_id: i64, language_id: i64, text: String, description: String) -> Translation {
        Translation { id: 0, card_id, language_id, text, description }
    }

    /// Save a Translation to database (insert or update)
    pub fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let mut values = vec![
            sqlite::Value::Integer(self.card_id),
            sqlite::Value::Integer(self.language_id),
            sqlite::Value::String(self.text.clone()),
            sqlite::Value::String(self.description.clone()),
        ];
        if self.id > 0 {
            values.push(sqlite::Value::Integer(self.id));
            Translation::update(conn, &values)?;
        } else {
            self.id = Translation::insert(conn, &values)?;
        }
        Ok(self.id)
    }
}


impl Model for Translation {
    const TABLE_NAME: &'static str = "translation";
    const STATEMENT_INSERT: &'static str =
        "INSERT INTO translation (card_id, language_id, text, description) VALUES (?, ?, ?, ?)";
    const STATEMENT_SELECT: &'static str =
        "SELECT id, card_id, language_id, text, description FROM translation WHERE id = ?";
    const STATEMENT_SELECT_ALL: &'static str =
        "SELECT id, card_id, language_id, text, description FROM translation ORDER BY id";
    const STATEMENT_UPDATE: &'static str =
        "UPDATE translation SET card_id = ?, language_id = ?, text = ?, description = ? WHERE id = ?";

    fn from_empty() -> Translation {
        Translation {
            id: 0, card_id: 0, language_id: 0, text: "".to_string(), description: "".to_string()
        }
    }

    fn from_row(row: &[sqlite::Value]) -> Result<Translation, DatabaseError> {
        let id = match row[0].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let card_id = match row[1].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let language_id = match row[2].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let text = match row[3].as_string() {
            Some(code) => code.to_string(),
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let description = match row[4].as_string() {
            Some(name) => name.to_string(),
            None => { return Err(DatabaseError::ValueNotString); },
        };
        Ok(Translation { id, card_id, language_id, text, description})
    }
}