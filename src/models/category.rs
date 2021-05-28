//! Model Category

use sqlite;

use crate::database::{DatabaseError, last_insert_id};
use super::Model;


/// A flash card category
#[derive(Debug)]
pub struct Category {
    /// Identifier of the category
    pub id: i64,
    /// Name of the category
    pub name: String,
}


impl Category {
    /// Instantiate a new category
    #[allow(dead_code)]
    pub fn new(id: i64, name: String) -> Category {
        Category { id, name }
    }
}


impl Model for Category {
    /// Table name for Category
    const TABLE_NAME: &'static str = "category";

    /// Instantiate an empty Category
    fn from_empty() -> Category {
        Category { id: 0, name: "".to_string() }
    }
    /// Instantiate a Category from given SQLite row
    fn from_row(row: &[sqlite::Value]) -> Result<Category, DatabaseError> {
        let id = match row[0].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let name = match row[1].as_string() {
            Some(name) => name.to_string(),
            None => { return Err(DatabaseError::ValueNotString); },
        };
        Ok(Category { id, name })
    }

    /// Load a Category by given identifier from the database
    fn load(conn: &sqlite::Connection, id: i64) -> Result<Category, DatabaseError> {
        let statement = format!(
            "SELECT id, name from {} WHERE id = ?", Category::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let category = Category::from_row(row)?;
            return Ok(category);
        }
        Err(DatabaseError::NotFound)
    }

    /// Load all existing Category from database
    fn load_all(conn: &sqlite::Connection) -> Result<Vec<Category>, DatabaseError> {
        let statement = format!("Select id, name from {} ORDER BY id", Category::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        let mut categories = Vec::new();
        while let Some(row) = cursor.next()? {
            let category = Category::from_row(row)?;
            categories.push(category);
        }
        Ok(categories)
    }

    /// Save the Category to the database and set the id
    fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let statement = "INSERT INTO category (name) VALUES (?)";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[
            sqlite::Value::String(self.name.clone()),
        ])?;
        cursor.next()?;
        self.id = last_insert_id(conn, Category::TABLE_NAME)?;
        Ok(self.id)
    }
}