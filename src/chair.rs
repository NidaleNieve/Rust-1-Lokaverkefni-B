use crate::chair_type::ChairType;
use crate::location::Location;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chair {
    pub id: Option<i64>,
    pub location: Location,
    pub value: u32,         // Value in ISK
    pub chair_type: ChairType,
}

impl Chair {
    pub fn new(location: Location, value: u32, chair_type: ChairType) -> Self {
        Chair { id: None, location, value, chair_type }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }
}

impl TryFrom<(Location, u32, ChairType)> for Chair {
    type Error = String;

    fn try_from(value: (Location, u32, ChairType)) -> Result<Self, Self::Error> {
        let (location, value_isk, chair_type) = value;
        Ok(Chair::new(location, value_isk, chair_type))
    }
}

impl fmt::Display for Chair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = self.id {
            write!(
                f,
                "Stóll með ID: {}, kostar {} kr., af gerðinni {} og er staðsettur í {}",
                id, self.value, self.chair_type, self.location
            )
        } else {
            write!(
                f,
                "Stóll, kostar {} kr., af gerðinni {} og er staðsettur í {}",
                self.value, self.chair_type, self.location
            )
        }
    }
}
