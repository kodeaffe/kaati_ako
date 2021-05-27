//! Model Category

/// A flash card category
#[derive(Debug)]
pub struct Category {
    /// Identifier of the category
    pub id: i64,
    /// Name of the category
    pub name: String,
}