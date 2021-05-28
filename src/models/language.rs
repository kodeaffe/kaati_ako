//! Model Language

use sqlite;

use crate::database::{DatabaseError, last_insert_id};
use super::Model;


/// The language of a flash card translation
#[derive(Debug)]
pub struct Language {
    /// Identifier of the language
    pub id: i64,
    /// Code of the language as in [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    pub code: String,
    /// Name of the language
    pub name: String,
}

impl Language {
    /// Instantiate a new language
    #[allow(dead_code)]
    pub fn new() -> Language {
        Language::from_empty()
    }
}


impl Model for Language {
    /// Table name for Language
    const TABLE_NAME: &'static str = "language";

    /// Instantiate an empty Language
    fn from_empty() -> Language {
        Language { id: 0, code: "".to_string(), name: "".to_string() }
    }

    /// Construct a Language from given SQLite row
    fn from_row(row: &[sqlite::Value]) -> Result<Language, DatabaseError> {
        let id = match row[0].as_integer() {
            Some(id) => id,
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let code = match row[1].as_string() {
            Some(code) => code.to_string(),
            None => { return Err(DatabaseError::ValueNotInteger); },
        };
        let name = match row[2].as_string() {
            Some(name) => name.to_string(),
            None => { return Err(DatabaseError::ValueNotString); },
        };
        Ok(Language { id, code, name })
    }

    /// Load a Language by given identifier from database
    fn load(conn: &sqlite::Connection, id: i64) -> Result<Language, DatabaseError> {
        let statement = format!(
            "SELECT id, code, name from {} WHERE id = ?", Language::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[sqlite::Value::Integer(id)])?;
        while let Some(row) = cursor.next()? {
            let language = Language::from_row(row)?;
            return Ok(language);
        }
        Err(DatabaseError::NotFound)
    }

    /// Load all existing Languages from database
    fn load_all(conn: &sqlite::Connection) -> Result<Vec<Language>, DatabaseError> {
        let statement = format!(
            "Select id, code, name from {} ORDER BY id", Language::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        let mut languages = Vec::new();
        while let Some(row) = cursor.next()? {
            let language = Language::from_row(row)?;
            languages.push(language);
        }
        Ok(languages)
    }

    /// Save the Language to the database and set the id
    fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let statement = "INSERT INTO language (code, name) VALUES (?, ?)";
        let mut cursor = conn.prepare(statement)?.cursor();
        cursor.bind(&[
            sqlite::Value::String(self.code.clone()),
            sqlite::Value::String(self.name.clone()),
        ])?;
        cursor.next()?;
        self.id = last_insert_id(conn, Language::TABLE_NAME)?;
        Ok(self.id)
    }
}