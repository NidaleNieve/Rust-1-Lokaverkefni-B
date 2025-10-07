use crate::location::Location;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Option<i64>,
    pub location: Location,
    pub value: u32, // Value in ISK
    pub seats: u8,  // Number of seats
}

impl Table {
    pub fn new(location: Location, value: u32, seats: u8) -> Self {
        Table {
            id: None,
            location,
            value,
            seats,
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = self.id {
            write!(
                f,
                "Borð með ID: {}, kostar {} kr., fyrir {} manns og er staðsett í {}",
                id, self.value, self.seats, self.location
            )
        } else {
            write!(
                f,
                "Borð, kostar {} kr., fyrir {} manns og er staðsett í {}",
                self.value, self.seats, self.location
            )
        }
    }
}
