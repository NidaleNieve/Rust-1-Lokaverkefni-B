use serde::{Serialize, Deserialize};
use std::fmt;

use super::{TableItem, Chair, Projector, Location, ChairKind};

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
        match self.kind {
            EquipmentKind::Table => write!(
                f, "{}",
                TableItem {
                    id: self.id,
                    value_isk: self.value_isk,
                    location: self.location.clone(),
                    seats: self.seats.unwrap_or(0),
                }
            ),
            EquipmentKind::Chair => write!(
                f, "{}",
                Chair {
                    id: self.id,
                    value_isk: self.value_isk,
                    location: self.location.clone(),
                    kind: self.chair_kind.unwrap_or(ChairKind::Annad),
                }
            ),
            EquipmentKind::Projector => write!(
                f, "{}",
                Projector {
                    id: self.id,
                    value_isk: self.value_isk,
                    location: self.location.clone(),
                    lumens: self.lumens.unwrap_or(0),
                }
            ),
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
        if e.kind != EquipmentKind::Table { return Err("Ekki Table".into()); }
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
        if e.kind != EquipmentKind::Chair { return Err("Ekki Chair".into()); }
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
        if e.kind != EquipmentKind::Projector { return Err("Ekki Projector".into()); }
        Ok(Projector {
            id: e.id,
            value_isk: e.value_isk,
            location: e.location,
            lumens: e.lumens.ok_or("vantar lumens")?,
        })
    }
}