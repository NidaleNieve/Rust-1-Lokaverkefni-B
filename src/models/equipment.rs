use serde::{Deserialize, Serialize};
use std::fmt;
use thousands::Separable;

use super::{Chair, ChairKind, Location, Projector, TableItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipmentKind {
    Table,
    Chair,
    Projector,
}

impl EquipmentKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            EquipmentKind::Table => "Table",
            EquipmentKind::Chair => "Chair",
            EquipmentKind::Projector => "Projector",
        }
    }

    pub fn friendly_name(&self) -> &'static str {
        match self {
            EquipmentKind::Table => "Borð",
            EquipmentKind::Chair => "Stóll",
            EquipmentKind::Projector => "Skjávarpi",
        }
    }
}

impl fmt::Display for EquipmentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for EquipmentKind {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Table" | "table" => Ok(EquipmentKind::Table),
            "Chair" | "chair" => Ok(EquipmentKind::Chair),
            "Projector" | "projector" => Ok(EquipmentKind::Projector),
            _ => Err(format!("Óþekkt tegund: {}", value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EquipmentRecord {
    pub id: Option<i64>,
    pub kind: EquipmentKind,
    pub value_isk: i64,
    pub location: Location,
    pub seats: Option<u8>,
    pub chair_kind: Option<ChairKind>,
    pub lumens: Option<u32>,
}

impl fmt::Display for EquipmentRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_description())
    }
}

impl EquipmentRecord {
    pub fn pretty_description(&self) -> String {
        let id_text = self
            .id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "óskráð".into());
        let value_text = self.value_isk.max(0).separate_with_spaces();
        let location = self.location.to_string();

        match self.kind {
            EquipmentKind::Table => {
                let seats = self.seats.unwrap_or(0);
                let seat_phrase = if seats == 0 {
                    "með óskráðum sætum".to_string()
                } else {
                    format!(
                        "með {} {}",
                        seats,
                        if seats == 1 { "sæti" } else { "sætum" }
                    )
                };
                format!(
                    "Borð með ID: {}, kostar {} kr., {} og er staðsett í {}.",
                    id_text, value_text, seat_phrase, location
                )
            }
            EquipmentKind::Chair => {
                let kind = self
                    .chair_kind
                    .map(|k| k.to_string())
                    .unwrap_or_else(|| "óþekktri gerð".into());
                format!(
                    "Stóll með ID: {}, kostar {} kr., af gerð {} og er staðsettur í {}.",
                    id_text, value_text, kind, location
                )
            }
            EquipmentKind::Projector => {
                let lumens_text = self
                    .lumens
                    .map(|l| format!("með {} lumen", l.separate_with_spaces()))
                    .unwrap_or_else(|| "með óskráða lumen".into());
                format!(
                    "Skjávarpi með ID: {}, kostar {} kr., {} og er staðsettur í {}.",
                    id_text, value_text, lumens_text, location
                )
            }
        }
    }
}

// TryFrom implementations to go from the specific structs to EquipmentRecord and back:

impl From<TableItem> for EquipmentRecord {
    fn from(t: TableItem) -> Self {
        EquipmentRecord {
            id: t.id,
            kind: EquipmentKind::Table,
            value_isk: t.value_isk,
            location: t.location,
            seats: Some(t.seats),
            chair_kind: None,
            lumens: None,
        }
    }
}

impl TryFrom<EquipmentRecord> for TableItem {
    type Error = String;
    fn try_from(e: EquipmentRecord) -> Result<Self, Self::Error> {
        if e.kind != EquipmentKind::Table {
            return Err("Ekki Table".into());
        }
        Ok(TableItem {
            id: e.id,
            value_isk: e.value_isk,
            location: e.location,
            seats: e.seats.ok_or("vantar seats")?,
        })
    }
}

impl From<Chair> for EquipmentRecord {
    fn from(c: Chair) -> Self {
        EquipmentRecord {
            id: c.id,
            kind: EquipmentKind::Chair,
            value_isk: c.value_isk,
            location: c.location,
            seats: None,
            chair_kind: Some(c.kind),
            lumens: None,
        }
    }
}

impl TryFrom<EquipmentRecord> for Chair {
    type Error = String;
    fn try_from(e: EquipmentRecord) -> Result<Self, Self::Error> {
        if e.kind != EquipmentKind::Chair {
            return Err("Ekki Chair".into());
        }
        Ok(Chair {
            id: e.id,
            value_isk: e.value_isk,
            location: e.location,
            kind: e.chair_kind.ok_or("vantar chair_kind")?,
        })
    }
}

impl From<Projector> for EquipmentRecord {
    fn from(p: Projector) -> Self {
        EquipmentRecord {
            id: p.id,
            kind: EquipmentKind::Projector,
            value_isk: p.value_isk,
            location: p.location,
            seats: None,
            chair_kind: None,
            lumens: Some(p.lumens),
        }
    }
}

impl TryFrom<EquipmentRecord> for Projector {
    type Error = String;
    fn try_from(e: EquipmentRecord) -> Result<Self, Self::Error> {
        if e.kind != EquipmentKind::Projector {
            return Err("Ekki Projector".into());
        }
        Ok(Projector {
            id: e.id,
            value_isk: e.value_isk,
            location: e.location,
            lumens: e.lumens.ok_or("vantar lumens")?,
        })
    }
}
