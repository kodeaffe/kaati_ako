//! Model Card

use sqlite;

use crate::database::DatabaseError;
use crate::models::category::Category;
use crate::models::translation::Translation;
use super::Model;


/// A flash card
#[derive(Debug)]
pub struct Card {
    /// Identifier of the card
    pub id: i64,
    /// Identifier of the Card's category
    pub category_id: i64,

    /// Category of the card
    pub category: Category,

    /// Translations of the card
    pub translations: Vec<Translation>,
}


impl Card {
    /// Instantiate a new Card for given category
    pub fn new(category_id: i64) -> Card {
        let mut card = Card::from_empty();
        card.category_id = category_id;
        card
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
        Card {
            id: 0,
            category_id: 0,
            category: Category::from_empty(),
            translations: vec![Translation::from_empty()],
        }
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
        let mut card = Card::from_empty();
        card.id = id;
        card.category_id = category_id;
        Ok(card)
    }

    fn get_save_values(&self) -> Vec<sqlite::Value> {
        vec![sqlite::Value::Integer(self.category_id)]
    }
}