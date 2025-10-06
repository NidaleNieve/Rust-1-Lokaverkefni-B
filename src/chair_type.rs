use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChairType {
    Haegindastoll,   // Comfort chair
    Skolastoll,      // School chair
    Skrifstofustoll, // Office chair
    Annad,           // Other
}

impl ChairType {
    pub fn all() -> Vec<ChairType> {
        vec![
            ChairType::Haegindastoll,
            ChairType::Skolastoll,
            ChairType::Skrifstofustoll,
            ChairType::Annad,
        ]
    }
}

impl fmt::Display for ChairType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChairType::Haegindastoll => "Hægindastóll",
                ChairType::Skolastoll => "Skólastóll",
                ChairType::Skrifstofustoll => "Skrifstofustóll",
                ChairType::Annad => "Annað",
            }
        )
    }
}

impl TryFrom<&str> for ChairType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Hægindastóll" => Ok(ChairType::Haegindastoll),
            "Skólastóll" => Ok(ChairType::Skolastoll),
            "Skrifstofustóll" => Ok(ChairType::Skrifstofustoll),
            "Annað" => Ok(ChairType::Annad),
            _ => Err(format!("Óþekkt stólategund: {}", value)),
        }
    }
}

impl TryFrom<String> for ChairType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ChairType::try_from(value.as_str())
    }
}
