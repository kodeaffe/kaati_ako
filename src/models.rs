//! Contains various (database) models

use sqlite;

use crate::database::{DatabaseError, last_insert_id};


pub mod category;
pub mod card;
pub mod language;
pub mod translation;


/// A trait to implement a (database) model
pub trait Model {
    /// Table name used for database operations
    const TABLE_NAME: &'static str;

    /// SQL Statement to save one item to database
    const STATEMENT_INSERT: &'static str;

    /// SQL statement to load one item from database
    const STATEMENT_SELECT: &'static str;

    /// SQL Statement to load all items from database
    const STATEMENT_SELECT_ALL: &'static str;

    /// SQL Statement to update one item in the database
    const STATEMENT_UPDATE: &'static str;

    /// Delete one object from database by id
    ///
    /// Default implementation available
    fn delete(conn: &sqlite::Connection, id: i64) -> Result<bool, DatabaseError> where Self: Sized {
        let statement = format!("DELETE FROM {} where id = ?", Self::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        cursor.next()?;
        Ok(true)
    }

    /// Instantiate an empty object
    fn from_empty() -> Self;

    /// Instantiate an object from given SQLite row
    fn from_row(row: &[sqlite::Value]) -> Result<Self, DatabaseError>
        where Self: Sized;

    /// Insert an item into the database, returning the id; default implementation available
    ///
    /// Default implementation available
    ///
    /// `Self::STATEMENT_INSERT` is used to update the data and the given values must correspond to
    /// that statement.
    fn insert(
        conn: &sqlite::Connection, values: &Vec<sqlite::Value>,
    ) -> Result<i64, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_INSERT)?.cursor();
        cursor.bind(values)?;
        cursor.next()?;
        let id = last_insert_id(conn, Self::TABLE_NAME)?;
        Ok(id)
    }

    /// Load one object from database by id
    ///
    /// Default implementation available
    ///
    /// `Self::STATEMENT_SELECT` is used to load the data
    fn load(conn: &sqlite::Connection, id: i64) -> Result<Self, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_SELECT)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let item = Self::from_row(row)?;
            return Ok(item);
        }
        Err(DatabaseError::NotFound)
    }

    /// Load all objects from database
    ///
    /// Default implementation available
    ///
    /// `Self::STATEMENT_SELECT_ALL` is used to load the data
    fn load_all(conn: &sqlite::Connection) -> Result<Vec<Self>, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_SELECT_ALL)?.cursor();
        let mut items = Vec::new();
        while let Some(row) = cursor.next()? {
            let item = Self::from_row(row)?;
            items.push(item);
        }
        Ok(items)
    }

    /// Update an existing object in the database
    ///
    /// Default implementation available
    ///
    /// `Self::STATEMENT_UPDATE` is used to update the data and the given values must correspond to
    /// that statement.
    fn update(
        conn: &sqlite::Connection,
        values: &Vec<sqlite::Value>,
    ) -> Result<bool, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_UPDATE)?.cursor();
        cursor.bind(values)?;
        cursor.next()?;
        Ok(true)
    }
}