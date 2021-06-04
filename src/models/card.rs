//! Model Card

use sqlite;

use crate::database::DatabaseError;
use super::Model;


/// A flash card
#[derive(Debug)]
pub struct Card {
    /// Identifier of the card
    pub id: i64,
    /// Identifier of the Card's category
    pub category_id: i64,
}


impl Card {
    /// Get a card with given id from database
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    /// * `card_id` - Identifier of the card to get. Will get a random card if 0.
    pub fn get(conn: &sqlite::Connection, card_id: i64) -> Result<Card, DatabaseError> {
        let id = if card_id == 0 { Card::random_id(&conn)? } else { card_id };
        let card = Card::load(conn, id)?;
        Ok(card)
    }

    /// Get the id of a random Card
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
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

    /// Save a Card to database (insert or update)
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    pub fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let mut values = vec![sqlite::Value::Integer(self.category_id)];
        if self.id > 0 {
            values.push(sqlite::Value::Integer(self.id));
            Card::update(conn, &values)?;
        } else {
            self.id = Card::insert(conn, &values)?;
        }
        Ok(self.id)
    }
}


impl Model for Card {
    const TABLE_NAME: &'static str = "card";
    const STATEMENT_INSERT: &'static str = "INSERT INTO card (category_id) VALUES (?)";
    const STATEMENT_SELECT: &'static str = "SELECT id, category_id FROM card WHERE id = ?";
    const STATEMENT_SELECT_ALL: &'static str = "SELECT id, category_id FROM card ORDER BY id";
    const STATEMENT_UPDATE: &'static str = "UPDATE card SET category_id = ? WHERE id = ?";

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
        let mut card = Card::from_empty();
        card.id = id;
        card.category_id = category_id;
        Ok(card)
    }
}