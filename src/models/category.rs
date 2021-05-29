//! Model Category

use sqlite;

use crate::database::DatabaseError;
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
    const TABLE_NAME: &'static str = "category";
    const STATEMENT_LOAD: &'static str = "SELECT id, name FROM category WHERE id = ?";
    const STATEMENT_LOAD_ALL: &'static str = "SELECT id, name FROM category ORDER BY name";
    const STATEMENT_SAVE: &'static str = "INSERT INTO category (name) VALUES (?)";

    fn from_empty() -> Category {
        Category { id: 0, name: "".to_string() }
    }

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

    fn get_save_values(&self) -> Vec<sqlite::Value> {
        vec![sqlite::Value::String(self.name.clone())]
    }
}