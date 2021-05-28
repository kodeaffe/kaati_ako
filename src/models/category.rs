//! Model Category

use sqlite;

use crate::database::DatabaseError;


/// A flash card category
#[derive(Debug)]
pub struct Category {
    /// Identifier of the category
    pub id: i64,
    /// Name of the category
    pub name: String,
}


impl Category {
    /// Table name for categories
    const TABLE_NAME: &'static str = "category";

    /// Instantiate an empty category
    pub fn from_empty() -> Category {
        Category { id: 0, name: "".to_string() }
    }

    /// Load a category by given identifier from the database
    pub fn load(conn: &sqlite::Connection, id: i64) -> Result<Category, DatabaseError> {
        let statement = format!(
            "SELECT id, name from {} WHERE id = ?", Category::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let id = match row[0].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            let name = match row[1].as_string() {
                Some(name) => name.to_string(),
                None => { return Err(DatabaseError::ValueNotString); },
            };
            return Ok(Category { id, name });
        }
        Err(DatabaseError::Unexpected)
    }

    /// Instantiate a new category
    #[allow(dead_code)]
    pub fn new() -> Category {
        Category::from_empty()
    }
}