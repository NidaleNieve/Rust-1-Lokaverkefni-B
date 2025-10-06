use crate::db;
use crate::models::*;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub struct Inventory {
    conn: Connection,
}

impl Inventory {
    pub fn open(path: &str) -> Result<Self, db::DbError> {
        let conn = db::open_or_init(path)?;
        Ok(Self { conn })
    }

    pub fn add(&self, item: EquipmentRecord) -> Result<i64, db::DbError> {
        db::insert_equipment(&self.conn, &item)
    }
    pub fn remove(&self, id: i64) -> Result<bool, db::DbError> {
        db::delete_equipment(&self.conn, id)
    }
    pub fn update_location(&self, id: i64, loc: &Location) -> Result<bool, db::DbError> {
        db::update_location(&self.conn, id, loc)
    }
    pub fn by_id(&self, id: i64) -> Result<Option<EquipmentRecord>, db::DbError> {
        db::get_by_id(&self.conn, id)
    }
    pub fn all(&self) -> Result<Vec<EquipmentRecord>, db::DbError> {
        db::list_all(&self.conn)
    }
    pub fn by_house(&self, h: House) -> Result<Vec<EquipmentRecord>, db::DbError> {
        db::list_by_house(&self.conn, h)
    }
    pub fn by_kind(&self, k: EquipmentKind) -> Result<Vec<EquipmentRecord>, db::DbError> {
        db::list_by_kind(&self.conn, k)
    }
    pub fn by_room(&self, loc: &Location) -> Result<Vec<EquipmentRecord>, db::DbError> {
        db::list_by_room(&self.conn, loc)
    }
    pub fn by_floor(&self, house: House, floor: u8) -> Result<Vec<EquipmentRecord>, db::DbError> {
        db::list_by_floor(&self.conn, house, floor)
    }

    // JSON Export/Import
    pub fn export_json(&self, path: &str) -> Result<usize, std::io::Error> {
        let items = self
            .all()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let dump = JsonDump { version: 1, items };
        let data = serde_json::to_string_pretty(&dump).unwrap();
        std::fs::write(path, data)?;
        Ok(dump.items.len())
    }

    pub fn import_json(&self, path: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let dump: JsonDump = serde_json::from_str(&data)?;
        let mut count = 0usize;
        for mut rec in dump.items {
            rec.id = None; // new ids on import
            let _ = self.add(rec)?;
            count += 1;
        }
        Ok(count)
    }
}

#[derive(Serialize, Deserialize)]
struct JsonDump {
    version: u32,
    items: Vec<EquipmentRecord>,
}
