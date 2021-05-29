//! Model Card

use sqlite;

use crate::database::DatabaseError;
use super::Model;


/// A flash card
#[derive(Debug)]
pub struct Card {
    /// Identifier of the card
    pub id: i64,
    /// Category of the card
    pub category_id: i64,
}


impl Card {
    /// Instantiate a new Card for given category
    pub fn new(category_id: i64) -> Card {
        Card { id: 0, category_id }
    }

    /// Get the id of a random Card
    pub fn random_id(conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let statement = format!(
            "SELECT id FROM {} ORDER BY RANDOM() LIMIT 1", Card::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        while let Some(row) = cursor.next()? {
            return match row[0].as_integer() {
                Some(id) => Ok(id),
                None => Err(DatabaseError::ValueNotInteger),
            }
        }
        Err(DatabaseError::NotFound)
    }
}


impl Model for Card {
    const TABLE_NAME: &'static str = "card";
    const STATEMENT_LOAD: &'static str = "SELECT id, category_id FROM card WHERE id = ?";
    const STATEMENT_LOAD_ALL: &'static str = "SELECT id, category_id FROM card ORDER BY id";
    const STATEMENT_SAVE: &'static str = "INSERT INTO card (category_id) VALUES (?)";

    fn from_empty() -> Card {
        Card { id: 0, category_id: 0 }
    }

    fn from_row(row: &[sqlite::Value]) -> Result<Card, DatabaseError> {
        let id = match row[0].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let category_id = match row[1].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        Ok(Card { id, category_id })
    }

    fn get_save_values(&self) -> Vec<sqlite::Value> {
        vec![sqlite::Value::Integer(self.category_id)]
    }
}