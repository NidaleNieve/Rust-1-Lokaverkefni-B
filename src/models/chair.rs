use serde::{Serialize, Deserialize};
use std::fmt;
use super::{Location, ChairKind};
use thousands::Separable;

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
            "Stóll með ID: {}, gerð: {}, kostar {}kr, staðsettur í {}",
            self.id.map(|x| x.to_string()).unwrap_or_else(|| "-".into()),
            self.kind,
            self.value_isk.separate_with_spaces(),
            self.location
        )
    }
}