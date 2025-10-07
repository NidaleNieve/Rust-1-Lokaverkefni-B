use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub building: Building,
    pub floor: u8,
    pub room: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Building {
    Hafnarfjordur,  // HA
    Hateigssvegur,  // H
    Skolavorduhollt, // S
}

impl Building {
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "HA" => Some(Building::Hafnarfjordur),
            "H" => Some(Building::Hateigssvegur),
            "S" => Some(Building::Skolavorduhollt),
            _ => None,
        }
    }

    pub fn to_code(&self) -> &str {
        match self {
            Building::Hafnarfjordur => "HA",
            Building::Hateigssvegur => "H",
            Building::Skolavorduhollt => "S",
        }
    }

    pub fn all() -> Vec<Building> {
        vec![
            Building::Hafnarfjordur,
            Building::Hateigssvegur,
            Building::Skolavorduhollt,
        ]
    }
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Building::Hafnarfjordur => "Hafnarfjörður",
                Building::Hateigssvegur => "Háteigsvegur",
                Building::Skolavorduhollt => "Skólavörðuholt",
            }
        )
    }
}

impl TryFrom<&str> for Building {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Accept both short codes and full Icelandic names
        match value.trim() {
            "HA" | "Hafnarfjörður" | "Hafnarfjordur" => Ok(Building::Hafnarfjordur),
            "H" | "Háteigsvegur" | "Hateigssvegur" => Ok(Building::Hateigssvegur),
            "S" | "Skólavörðuholt" | "Skolavorduhollt" => Ok(Building::Skolavorduhollt),
            other => Err(format!("Óþekkt hús: {}", other)),
        }
    }
}

impl TryFrom<String> for Building {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Building::try_from(value.as_str())
    }
}

impl Location {
    pub fn new(building: Building, floor: u8, room: u8) -> Self {
        Location {
            building,
            floor,
            room,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}{}", self.building.to_code(), self.floor, self.room)
    }
}

impl TryFrom<&str> for Location {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^(HA|H|S)-([0-9])([0-9]{1,2})$").unwrap();

        if let Some(caps) = re.captures(value) {
            let building_code = caps.get(1).unwrap().as_str();
            let floor_str = caps.get(2).unwrap().as_str();
            let room_str = caps.get(3).unwrap().as_str();

            let building = Building::from_code(building_code)
                .ok_or_else(|| format!("Ógilt húsmerki: {}", building_code))?;

            let floor = floor_str
                .parse::<u8>()
                .map_err(|_| format!("Ógild hæð: {}", floor_str))?;

            let room = room_str
                .parse::<u8>()
                .map_err(|_| format!("Ógillt herbergisnúmer: {}", room_str))?;

            if room > 99 {
                return Err(format!("Herbergisnúmer má ekki vera hærra en 99: {}", room));
            }

            Ok(Location {
                building,
                floor,
                room,
            })
        } else {
            Err(format!(
                "Ógilt staðsetningarsnið: {}. Ætti að vera t.d. H-202 eða HA-123",
                value
            ))
        }
    }
}

impl TryFrom<String> for Location {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Location::try_from(value.as_str())
    }
}

impl TryFrom<(Building, u8, u8)> for Location {
    type Error = String;

    fn try_from(value: (Building, u8, u8)) -> Result<Self, Self::Error> {
        let (building, floor, room) = value;
        if room > 99 {
            return Err(format!("Herbergisnúmer má ekki vera hærra en 99: {}", room));
        }
        Ok(Location { building, floor, room })
    }
}
