use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

use super::House;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub house: House,
    pub floor: u8,
    pub room: u16,
}

impl Location {
    pub fn code(&self) -> String {
        // H-202 => H, floor=2, room=2 (02 shown)
        format!("{}-{}{:02}", self.house, self.floor, self.room)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for Location {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept "HA-123" or "H-202" etc.
        let s = s.trim();
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 { return Err("Vænt formað sem HA-123".into()); }
        let house = parts[0].parse::<House>().map_err(|e: String| e)?;
        let digits = parts[1];
        if digits.len() < 2 { return Err("Eftir '-' þarf að vera a.m.k. tvær tölur".into()); }
        let floor = digits.chars().next().unwrap();
        if !floor.is_ascii_digit() { return Err("Hæð þarf að vera tala (eitt staf).".into()); }
        let floor = floor.to_digit(10).unwrap() as u8;
        let room_str = &digits[1..];
        let room: u16 = room_str.parse().map_err(|_| "Herbergisnúmer ógilt".to_string())?;
        Ok(Self { house, floor, room })
    }
}