//! Model Language

use sqlite;

use crate::database::DatabaseError;

/// The language of a flash card translation
#[derive(Clone, Debug)]
pub struct Language {
    /// Identifier of the language
    pub id: i64,
    /// Code of the language as in [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    pub code: String,
    /// Name of the language
    pub name: String,
}

impl Language {
    /// Table name for languages
    const TABLE_NAME: &'static str = "language";

    /// Instantiate an empty language
    pub fn from_empty() -> Language {
        Language { id: 0, code: "".to_string(), name: "".to_string() }
    }

    /// Load all existing languages from database
    pub fn load_all(conn: &sqlite::Connection) -> Result<Vec<Language>, DatabaseError> {
        let statement = format!(
            "Select id, code, name from {} ORDER BY id", Language::TABLE_NAME);
        let mut cursor = conn.prepare(statement)?.cursor();
        let mut languages = Vec::new();
        while let Some(row) = cursor.next()? {
            let id = match row[0].as_integer() {
                Some(id) => id,
                None => { return Err(DatabaseError::ValueNotInteger); },
            };
            let code = match row[1].as_string() {
                Some(code) => code.to_string(),
                None => { return Err(DatabaseError::ValueNotString); },
            };
            let name = match row[2].as_string() {
                Some(name) => name.to_string(),
                None => { return Err(DatabaseError::ValueNotString); },
            };
            languages.push(Language { id, code, name });
        }
        Ok(languages)
    }

    /// Instantiate a new language
    #[allow(dead_code)]
    pub fn new() -> Language {
        Language::from_empty()
    }
}