//! Model Card

use sqlite;

use crate::database::{DatabaseError, last_insert_id};
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
    /// Table name for Card
    const TABLE_NAME: &'static str = "card";

    /// Instantiate an empty Card
    fn from_empty() -> Card {
        Card { id: 0, category_id: 0 }
    }

    /// Construct a Card from given SQLite row
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

    /// Load a Card with translations by given identifier from database
    fn load(conn: &sqlite::Connection, id: i64) -> Result<Card, DatabaseError> {
        let statement = format!(
            "SELECT id, category_id FROM {} WHERE card.id = ?", Card::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let card = Card::from_row(row)?;
            return Ok(card);
        }
        Err(DatabaseError::NotFound)
    }

    /// Load all Cards with translations from database
    fn load_all(conn: &sqlite::Connection) -> Result<Vec<Card>, DatabaseError> {
        let statement = format!(
            "SELECT id, category_id FROM {} ORDER BY card.id = ?", Card::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        let mut cards = Vec::new();
        while let Some(row) = cursor.next()? {
            let card = Card::from_row(row)?;
            cards.push(card);
        }
        Ok(cards)
    }

    /// Save the Card to the database and set the id
    fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let statement = "INSERT INTO card (category_id) VALUES (?)";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(self.category_id)])?;
        cursor.next()?;
        self.id = last_insert_id(conn, Card::TABLE_NAME)?;
        Ok(self.id)
    }
}