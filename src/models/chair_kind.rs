use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChairKind {
    Haegindastoll,
    Skolastoll,
    Skrifstofustoll,
    Annad,
}

impl ChairKind {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            ChairKind::Haegindastoll => "Haegindastoll",
            ChairKind::Skolastoll => "Skolastoll",
            ChairKind::Skrifstofustoll => "Skrifstofustoll",
            ChairKind::Annad => "Annad",
        }
    }
    pub fn try_from_db_str(s: &str) -> Result<Self, String> {
        match s {
            "Haegindastoll" => Ok(ChairKind::Haegindastoll),
            "Skolastoll" => Ok(ChairKind::Skolastoll),
            "Skrifstofustoll" => Ok(ChairKind::Skrifstofustoll),
            "Annad" => Ok(ChairKind::Annad),
            _ => Err(format!("Ógilt chair_kind: {}", s)),
        }
    }
}

impl fmt::Display for ChairKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ChairKind::Haegindastoll => "Hægindastóll",
            ChairKind::Skolastoll => "Skólastóll",
            ChairKind::Skrifstofustoll => "Skrifstofustóll",
            ChairKind::Annad => "Annað",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ChairKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.trim().to_lowercase();
        match norm.as_str() {
            "hægindastóll" | "haegindastoll" | "hægi" | "haegi" => Ok(ChairKind::Haegindastoll),
            "skólastóll" | "skolastoll" => Ok(ChairKind::Skolastoll),
            "skrifstofustóll" | "skrifstofustoll" | "skrifsto" => Ok(ChairKind::Skrifstofustoll),
            "annað" | "annad" | "other" => Ok(ChairKind::Annad),
            _ => Err(format!("Ógild gerð stóls: {}", s)),
        }
    }
}