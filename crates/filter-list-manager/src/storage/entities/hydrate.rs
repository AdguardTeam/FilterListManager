use rusqlite::{Result, Row};

/// Trait for hydrating entities from database
pub trait Hydrate: Sized {
    /// Hydrate entity from database row
    fn hydrate(row: &Row) -> Result<Self>;
}
