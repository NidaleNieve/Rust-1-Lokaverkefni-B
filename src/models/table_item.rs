use serde::{Deserialize, Serialize};
use std::fmt;

use super::Location;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableItem {
    pub id: Option<i64>,
    pub value_isk: i64,
    pub location: Location,
    pub seats: u8,
}

impl fmt::Display for TableItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Borð(id={}, sæti={}, verðmæti={} kr., staðsetning={})",
            self.id.map(|x| x.to_string()).unwrap_or_else(|| "-".into()),
            self.seats,
            self.value_isk,
            self.location
        )
    }
}
