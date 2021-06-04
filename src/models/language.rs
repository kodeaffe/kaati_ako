//! Model Language

use sqlite;

use crate::database::DatabaseError;
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
    /// Save a Language to database (insert or update)
    ///
    /// # Arguments
    ///
    /// * `conn` - Connection to the database
    #[allow(dead_code)]
    pub fn save(&mut self, conn: &sqlite::Connection) -> Result<i64, DatabaseError> {
        let mut values = vec![
            sqlite::Value::String(self.code.clone()),
            sqlite::Value::String(self.name.clone()),
        ];
        if self.id > 0 {
            values.push(sqlite::Value::Integer(self.id));
            Language::update(conn, &values)?;
        } else {

            self.id = Language::insert(conn, &values)?;
        }
        Ok(self.id)
    }
}


impl Model for Language {
    const TABLE_NAME: &'static str = "language";
    const STATEMENT_INSERT: &'static str = "INSERT INTO language (code, name) VALUES (?, ?)";
    const STATEMENT_SELECT: &'static str = "SELECT id, code, name FROM language WHERE id = ?";
    const STATEMENT_SELECT_ALL: &'static str = "SELECT id, code, name FROM language ORDER BY name";
    const STATEMENT_UPDATE: &'static str = "UPDATE language SET code = ?, name = ? WHERE id = ?";

    fn from_empty() -> Language {
        Language { id: 0, code: "".to_string(), name: "".to_string() }
    }

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
}