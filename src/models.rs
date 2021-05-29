//! Contains various (database) models

use sqlite;

use crate::database::{DatabaseError, last_insert_id};


pub mod category;
pub mod card;
pub mod language;
pub mod translation;


/// A trait to implement a (database) model
pub trait Model {
    /// Table name for translations
    const TABLE_NAME: &'static str;

    /// SQL statement to load one item from database
    const STATEMENT_LOAD: &'static str;

    /// SQL Statement to load all items from database
    const STATEMENT_LOAD_ALL: &'static str;

    /// SQL Statement to save one item to database
    const STATEMENT_SAVE: &'static str;

    /// Instantiate an empty object
    fn from_empty() -> Self;

    /// Instantiate an object from given SQLite row
    fn from_row(row: &[sqlite::Value]) -> Result<Self, DatabaseError>
        where Self: Sized;

    /// Load one object from database by id; default implementation available
    fn load(conn: &sqlite::Connection, id: i64) -> Result<Self, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_LOAD)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let item = Self::from_row(row)?;
            return Ok(item);
        }
        Err(DatabaseError::NotFound)
    }

    /// Load all objects from database; default implementation available
    fn load_all(conn: &sqlite::Connection) -> Result<Vec<Self>, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_LOAD_ALL)?.cursor();
        let mut items = Vec::new();
        while let Some(row) = cursor.next()? {
            let item = Self::from_row(row)?;
            items.push(item);
        }
        Ok(items)
    }

    /// Save an object to the database, returning the database id; default implementation available.
    ///
    /// Alas, traits do now allow to modify fields of Self, so we just return the id and the caller
    /// needs to assign the id in the object.
    ///
    /// # Example
    /// ```rust
    ///     match item.save(&conn) {
    ///         Ok(id) => item.id = id,
    ///         Err(err) => handle_error(&err),
    ///     }
    /// ```
    fn save(&self, conn: &sqlite::Connection) -> Result<i64, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_SAVE)?.cursor();
        cursor.bind(&self.get_save_values())?;
        cursor.next()?;
        let id = last_insert_id(conn, Self::TABLE_NAME)?;
        Ok(id)
    }

    /// Get values to bind to the SQL save statement
    fn get_save_values(&self) -> Vec<sqlite::Value>;
}