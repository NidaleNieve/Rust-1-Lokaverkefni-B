use crate::location::Location;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projector {
    pub id: Option<i64>,
    pub location: Location,
    pub value: u32, // Value in ISK
    pub lumens: u32,
}

impl Projector {
    pub fn new(location: Location, value: u32, lumens: u32) -> Self {
        Projector { id: None, location, value, lumens }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
}

impl TryFrom<(Location, u32, u32)> for Projector {
    type Error = String;

    fn try_from(value: (Location, u32, u32)) -> Result<Self, Self::Error> {
        let (location, value_isk, lumens) = value;
        if lumens == 0 { return Err("Lúmens má ekki vera 0".into()); }
        Ok(Projector::new(location, value_isk, lumens))
    }
}

impl fmt::Display for Projector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = self.id {
            write!(
                f,
                "Skjávarpi með ID: {}, kostar {} kr., með {} lúmens og er staðsettur í {}",
                id, self.value, self.lumens, self.location
            )
        } else {
            write!(
                f,
                "Skjávarpi, kostar {} kr., með {} lúmens og er staðsettur í {}",
                self.value, self.lumens, self.location
            )
        }
    }
}
