//! Contains various (database) models

use sqlite;

use crate::database::DatabaseError;


pub mod category;
pub mod card;
pub mod language;
pub mod translation;


/// A trait to implement a (database) model
pub trait Model {
    /// Table name for translations
    const TABLE_NAME: &'static str;

    /// Instantiate an empty object
    fn from_empty() -> Self;

    /// Instantiate an object from given SQLite row
    fn from_row(row: &[sqlite::Value]) -> Result<Self, DatabaseError>
        where Self: Sized;

    /// Load one object from database by id
    fn load(conn: &sqlite::Connection, id: i64) -> Result<Self, DatabaseError>
        where Self: Sized;

    /// Load all objects from database
    fn load_all(conn: &sqlite::Connection) -> Result<Vec<Self>, DatabaseError>
        where Self: Sized;

    /// Save an object to the database, returning the database id
    fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError>
        where Self: Sized;
}