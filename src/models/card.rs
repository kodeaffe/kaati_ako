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


impl Card {
    /// Default select statement to query (a) card
    const STATEMENT_SELECT: &'static str = "
        SELECT card.id, category.id, category.name
        FROM card
        LEFT JOIN category ON card.category_id = category.id
    ";

    /// Table name for cards
    const TABLE_NAME: &'static str = "card";

    /// Instantiate an empty card
    pub fn from_empty() -> Card {
        Card {
            id: 0,
            category: Category::from_empty(),
            translations: Vec::new(),
        }
    }

    /// Construct a Card from given SQLite row; used internally
    fn from_row(conn: &sqlite::Connection, row: &[sqlite::Value]) -> Result<Card, DatabaseError> {
        let card_id = match row[0].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let category_id = match row[1].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let category_name = match row[2].as_string() {
            Some(name) => name.to_string(),
            None => { return Err(DatabaseError::ValueNotString); },
        };
        let card = Card {
            id: card_id,
            category: Category { id: category_id, name: category_name },
            translations: Translation::load_for_card(conn, card_id)?,
        };
        Ok(card)
    }

    /// Load all flash cards with translations from database
    #[allow(dead_code)]
    pub fn load_all(conn: &sqlite::Connection) -> Result<Vec<Card>, DatabaseError> {
        let statement = format!("{} ORDER BY card.id", Card::STATEMENT_SELECT);
        let mut cursor = conn.prepare(statement)?.cursor();
        let mut cards = Vec::new();
        while let Some(row) = cursor.next()? {
            let card = Card::from_row(conn, row)?;
            cards.push(card);
        }
        Ok(cards)
    }

    /// Load a card with translations by given identifier from database
    pub fn load(conn: &sqlite::Connection, id: i64) -> Result<Card, DatabaseError> {
        let statement = format!("{} WHERE card.id = ?", Card::STATEMENT_SELECT);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let card = Card::from_row(conn, row)?;
            return Ok(card);
        }
        Ok(Card::from_empty())
    }

    /// Get the id of a random card
    pub fn random_id(conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let statement = format!(
            "SELECT id from {} ORDER BY RANDOM() LIMIT 1", Card::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        while let Some(row) = cursor.next()? {
            return match row[0].as_integer() {
                Some(id) => Ok(id),
                None => Err(DatabaseError::ValueNotInteger),
            }
        }
        Err(DatabaseError::Unexpected)
    }

    /// Instantiate a new card for given category
    pub fn new(category: Category) -> Card {
        let mut card = Card::from_empty();
        card.category = category;
        card
    }

    /// Save the card to the database and set the id
    pub fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let statement = "INSERT INTO card (category_id) VALUES (?)";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(self.category.id)])?;
        cursor.next()?;
        self.id = last_insert_id(conn, Card::TABLE_NAME)?;
        for translation in &mut self.translations {
            translation.save(conn, self.id)?;
        }
        Ok(self.id)
    }
}