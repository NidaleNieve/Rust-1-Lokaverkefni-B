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

    pub fn set_id(&mut self, id: i64) {
        match self {
            Equipment::Table(t) => t.id = Some(id),
            Equipment::Chair(c) => c.id = Some(id),
            Equipment::Projector(p) => p.id = Some(id),
        }
    }

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
