use crate::chair::Chair;
use crate::projector::Projector;
use crate::table::Table;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Equipment {
    Table(Table),
    Chair(Chair),
    Projector(Projector),
}

impl Equipment {
    pub fn get_id(&self) -> Option<i64> {
        match self {
            Equipment::Table(t) => t.id,
            Equipment::Chair(c) => c.id,
            Equipment::Projector(p) => p.id,
        }
    }

    /// Set the ID (useful for tests and JSON import flows)
    /// Kept public for tooling; harmless if unused in production builds.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn set_id(&mut self, id: i64) {
        match self {
            Equipment::Table(t) => t.id = Some(id),
            Equipment::Chair(c) => c.id = Some(id),
            Equipment::Projector(p) => p.id = Some(id),
        }
    }

// Unit tests for Equipment
// tests module moved to the bottom of the file (outside impl)

    pub fn get_type_name(&self) -> &str {
        match self {
            Equipment::Table(_) => "Borð",
            Equipment::Chair(_) => "Stóll",
            Equipment::Projector(_) => "Skjávarpi",
        }
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Equipment::Table(t) => write!(f, "{}", t),
            Equipment::Chair(c) => write!(f, "{}", c),
            Equipment::Projector(p) => write!(f, "{}", p),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{table::Table, location::{Location, Building}};

    #[test]
    fn can_set_id() {
        let loc = Location { building: Building::Hafnarfjordur, floor: 1, room: 1 };
        let mut eq = Equipment::Table(Table { id: None, location: loc, value: 1000, seats: 4 });
        eq.set_id(42);
        assert_eq!(eq.get_id(), Some(42));
    }
}
