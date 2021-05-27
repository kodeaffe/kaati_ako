//! Model Language

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
