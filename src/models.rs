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

    /// Delete one item from database by id
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    /// * `id` - Identifier of the item to delete
    fn delete(conn: &sqlite::Connection, id: i64) -> Result<bool, DatabaseError> where Self: Sized {
        let statement = format!("DELETE FROM {} where id = ?", Self::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        cursor.next()?;
        Ok(true)
    }

    /// Instantiate an 'empty' item
    ///
    /// 'Empty' means all fields are properly initialised.
    fn from_empty() -> Self where Self: Sized;

    /// Instantiate an item from given row
    ///
    /// # Arguments
    ///
    /// * `row` - Values retrieved from the database
    fn from_row(row: &[sqlite::Value]) -> Result<Self, DatabaseError> where Self: Sized;

    /// Insert an item into the database, returning the id; default implementation available
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    /// * `values` - Values to insert into the database, they must correspond to
    /// `Self::STATEMENT_INSERT`
    ///
    /// # Notes
    ///
    /// * `Self::STATEMENT_INSERT` is used to update the data
    fn insert(
        conn: &sqlite::Connection, values: &Vec<sqlite::Value>,
    ) -> Result<i64, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_INSERT)?.cursor();
        cursor.bind(values)?;
        cursor.next()?;
        let id = last_insert_id(conn, Self::TABLE_NAME)?;
        Ok(id)
    }

    /// Load one item from database by id
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    /// * `id` - Identifier of the item to load
    ///
    /// # Notes
    ///
    /// * Related items are not loaded automatically!
    /// * `Self::STATEMENT_SELECT` is used to load the data
    fn load(conn: &sqlite::Connection, id: i64) -> Result<Self, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_SELECT)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let item = Self::from_row(row)?;
            return Ok(item);
        }
        Err(DatabaseError::NotFound)
    }

    /// Load all items from database
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    ///
    /// # Notes
    ///
    /// * Related items are not loaded automatically!
    /// * `Self::STATEMENT_SELECT_ALL` is used to load the data
    fn load_all(conn: &sqlite::Connection) -> Result<Vec<Self>, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_SELECT_ALL)?.cursor();
        let mut items = Vec::new();
        while let Some(row) = cursor.next()? {
            let item = Self::from_row(row)?;
            items.push(item);
        }
        Ok(items)
    }

    /// Update an existing item in the database
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    /// * `values` - Values to update in the database, they must correspond to
    /// `Self::STATEMENT_UPDATE`
    ///
    /// # Notes
    ///
    /// * Related items are not updated automatically!
    /// * `Self::STATEMENT_UPDATE` is used to update the data
    fn update(
        conn: &sqlite::Connection,
        values: &Vec<sqlite::Value>,
    ) -> Result<bool, DatabaseError> where Self: Sized {
        let mut cursor = conn.prepare(Self::STATEMENT_UPDATE)?.cursor();
        cursor.bind(values)?;
        cursor.next()?;
        Ok(true)
    }

    // I wish a save function with the following signature to set the object's new id was possible:
    //fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError>
}