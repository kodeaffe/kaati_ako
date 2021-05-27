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
    /// Default select statement to query (a) card
    const STATEMENT_SELECT: &'static str = "
        SELECT card.id, category.id, category.name
        FROM card
        LEFT JOIN category ON card.category_id = category.id
    ";

    /// Table name for cards
    const TABLE_NAME: &'static str = "card";

    /// Add a new card to the database
    pub fn add(
        conn: &sqlite::Connection,
        category_id: i64,
        tongan: &str,
        english: &str,
        german: &str,
    ) -> Result<i64, DatabaseError> {
        //return Err(DatabaseError::SQLiteError("foo".to_string()));
        let card_id = Card::insert(conn, category_id)?;
        Translation::insert(conn, card_id, 1, tongan, "")?;
        Translation::insert(conn, card_id, 2, english, "")?;
        Translation::insert(conn, card_id, 3, german, "")?;
        Ok(card_id)
    }

    /// Instantiate an empty card
    pub fn from_empty() -> Card {
        Card {
            id: 0,
            category: Category { id: 0, name: "".to_string() },
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
            translations: Translation::get_all(conn, card_id)?,
        };
        Ok(card)
    }

    /// Select all flash cards with translations
    #[allow(dead_code)]
    pub fn get_all(conn: &sqlite::Connection) -> Result<Vec<Card>, DatabaseError> {
        let statement = format!("{} ORDER BY card.id", Card::STATEMENT_SELECT);
        let mut cursor = conn.prepare(statement)?.cursor();
        let mut cards = Vec::new();
        while let Some(row) = cursor.next()? {
            let card = Card::from_row(conn, row)?;
            cards.push(card);
        }
        Ok(cards)
    }

    /// Select a card with translations by given identifier
    pub fn get(conn: &sqlite::Connection, id: i64) -> Result<Card, DatabaseError> {
        let statement = format!("{} WHERE card.id = ?", Card::STATEMENT_SELECT);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let card = Card::from_row(conn, row)?;
            return Ok(card);
        }
        Ok(Card::from_empty())
    }

    /// Insert a card for given category into the database; used internally
    fn insert(conn: &sqlite::Connection, category_id: i64) -> Result<i64, DatabaseError> {
        let statement = "INSERT INTO card (category_id) VALUES (?)";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(category_id)])?;
        cursor.next()?;
        last_insert_id(conn, Card::TABLE_NAME)
    }

    /// Get the id of a random card
    pub fn random_id(conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let statement = format!(
            "SELECT card.id from {} ORDER BY RANDOM() LIMIT 1", Card::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        while let Some(row) = cursor.next()? {
            return match row[0].as_integer() {
                Some(id) => Ok(id),
                None => Err(DatabaseError::ValueNotInteger),
            }
        }
        Err(DatabaseError::Unexpected)
    }
}