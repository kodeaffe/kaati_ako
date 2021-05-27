//! Model Card

use sqlite;

use crate::database::{DatabaseError, last_insert_id};
use super::category::Category;
use super::translation::Translation;


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