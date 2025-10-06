use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum House { HA, H, S }

impl fmt::Display for House {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self { House::HA => "HA", House::H => "H", House::S => "S" })
    }
}

impl FromStr for House {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "HA" => Ok(House::HA),
            "H" => Ok(House::H),
            "S" => Ok(House::S),
            other => Err(format!("Invalid house: {}", other)),
        }
    }
}

impl Default for House {
    fn default() -> Self {
        House::H
    }
}