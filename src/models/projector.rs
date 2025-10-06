use serde::{Serialize, Deserialize};
use std::fmt;
use super::Location;
use thousands::Separable;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Projector {
    pub id: Option<i64>,
    pub value_isk: i64,
    pub location: Location,
    pub lumens: u32,
}

impl fmt::Display for Projector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Skjávarpi með ID: {}, kostar {}kr, með {} lumen og er staðsettur í {}",
            self.id.map(|x| x.to_string()).unwrap_or_else(|| "-".into()),
            self.value_isk.separate_with_spaces(),
            self.lumens,
            self.location
        )
    }
}