use serde::{Serialize, Deserialize};
use std::fmt;
use super::{Location, ChairKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chair {
    pub id: Option<i64>,
    pub value_isk: i64,
    pub location: Location,
    pub kind: ChairKind,
}

impl fmt::Display for Chair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stóll(id={}, gerð={}, verðmæti={} kr., staðsetning={})",
            self.id.map(|x| x.to_string()).unwrap_or_else(||"-".into()),
            self.kind, self.value_isk, self.location
        )
    }
}