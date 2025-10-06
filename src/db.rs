use rusqlite::{params, Connection, OptionalExtension};
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::models::{ChairKind, EquipmentKind, EquipmentRecord, House, Location};

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("SQLite villa: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Ógild gögn: {0}")]
    Invalid(String),
}

pub fn open_or_init(path: &str) -> Result<Connection, DbError> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).ok();
    }
    let conn = Connection::open(path)?;
    // Run migration
    let sql = include_str!("../migrations/001_init.sql");
    conn.execute_batch(sql)?;
    Ok(conn)
}

pub fn insert_equipment(conn: &Connection, rec: &EquipmentRecord) -> Result<i64, DbError> {
    // Validate domain rules
    match rec.kind {
        EquipmentKind::Table => {
            if rec.seats.is_none() {
                return Err(DbError::Invalid("Borð þarf seats".into()));
            }
        }
        EquipmentKind::Chair => {
            if rec.chair_kind.is_none() {
                return Err(DbError::Invalid("Stóll þarf chair_kind".into()));
            }
        }
        EquipmentKind::Projector => {
            if rec.lumens.is_none() {
                return Err(DbError::Invalid("Skjávarpi þarf lumens".into()));
            }
        }
    }

    let id: i64 = conn.query_row(
        "INSERT INTO equipment (kind, value_isk, house, floor, room, seats, chair_kind, lumens)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8) RETURNING id;",
        params![
            rec.kind.as_str(),
            rec.value_isk,
            rec.location.house.to_string(),
            rec.location.floor as i64,
            rec.location.room as i64,
            rec.seats.map(|s| s as i64),
            rec.chair_kind.map(|k| k.as_db_str().to_string()),
            rec.lumens.map(|l| l as i64),
        ],
        |row| row.get(0),
    )?;

    Ok(id)
}

pub fn delete_equipment(conn: &Connection, id: i64) -> Result<bool, DbError> {
    let n = conn.execute("DELETE FROM equipment WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn update_location(conn: &Connection, id: i64, loc: &Location) -> Result<bool, DbError> {
    let n = conn.execute(
        "UPDATE equipment SET house = ?1, floor = ?2, room = ?3 WHERE id = ?4",
        params![loc.house.to_string(), loc.floor as i64, loc.room as i64, id],
    )?;
    Ok(n > 0)
}

pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<EquipmentRecord>, DbError> {
    conn.query_row(
        "SELECT id, kind, value_isk, house, floor, room, seats, chair_kind, lumens
         FROM equipment WHERE id = ?1",
        params![id],
        |row| row_to_rec(row),
    )
    .optional()
    .map_err(Into::into)
}

pub fn list_all(conn: &Connection) -> Result<Vec<EquipmentRecord>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, kind, value_isk, house, floor, room, seats, chair_kind, lumens
         FROM equipment_sorted",
    )?;
    let it = stmt.query_map([], |row| row_to_rec(row))?;
    let mut out = Vec::new();
    for r in it {
        out.push(r?);
    }
    Ok(out)
}

pub fn list_by_house(conn: &Connection, house: House) -> Result<Vec<EquipmentRecord>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, kind, value_isk, house, floor, room, seats, chair_kind, lumens
         FROM equipment_sorted WHERE house = ?1",
    )?;
    let it = stmt.query_map(params![house.to_string()], |row| row_to_rec(row))?;
    Ok(it.collect::<Result<Vec<_>, _>>()?)
}

pub fn list_by_kind(
    conn: &Connection,
    kind: EquipmentKind,
) -> Result<Vec<EquipmentRecord>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, kind, value_isk, house, floor, room, seats, chair_kind, lumens
         FROM equipment_sorted WHERE kind = ?1",
    )?;
    let it = stmt.query_map(params![kind.as_str()], |row| row_to_rec(row))?;
    Ok(it.collect::<Result<Vec<_>, _>>()?)
}

pub fn list_by_room(conn: &Connection, loc: &Location) -> Result<Vec<EquipmentRecord>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, kind, value_isk, house, floor, room, seats, chair_kind, lumens
         FROM equipment_sorted WHERE house = ?1 AND floor = ?2 AND room = ?3",
    )?;
    let it = stmt.query_map(
        params![loc.house.to_string(), loc.floor as i64, loc.room as i64],
        |row| row_to_rec(row),
    )?;
    Ok(it.collect::<Result<Vec<_>, _>>()?)
}

pub fn list_by_floor(
    conn: &Connection,
    house: House,
    floor: u8,
) -> Result<Vec<EquipmentRecord>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, kind, value_isk, house, floor, room, seats, chair_kind, lumens
         FROM equipment_sorted WHERE house = ?1 AND floor = ?2",
    )?;
    let it = stmt.query_map(params![house.to_string(), floor as i64], |row| {
        row_to_rec(row)
    })?;
    Ok(it.collect::<Result<Vec<_>, _>>()?)
}

fn row_to_rec(row: &rusqlite::Row<'_>) -> rusqlite::Result<EquipmentRecord> {
    let id: i64 = row.get(0)?;
    let kind_s: String = row.get(1)?;
    let kind = EquipmentKind::try_from(kind_s.as_str()).unwrap_or(EquipmentKind::Table);
    let value_isk: i64 = row.get(2)?;
    let house_s: String = row.get(3)?;
    let floor_i: i64 = row.get(4)?;
    let room_i: i64 = row.get(5)?;
    let seats: Option<i64> = row.get(6)?;
    let chair_kind_s: Option<String> = row.get(7)?;
    let lumens: Option<i64> = row.get(8)?;

    let house = crate::models::House::from_str(&house_s).unwrap();
    let loc = Location {
        house,
        floor: floor_i as u8,
        room: room_i as u16,
    };

    Ok(EquipmentRecord {
        id: Some(id),
        kind,
        value_isk,
        location: loc,
        seats: seats.map(|x| x as u8),
        chair_kind: chair_kind_s
            .as_deref()
            .and_then(|s| ChairKind::try_from_db_str(s).ok()),
        lumens: lumens.map(|x| x as u32),
    })
}
