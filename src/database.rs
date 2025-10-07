use crate::chair::Chair;
use crate::chair_type::ChairType;
use crate::equipment::Equipment;
use crate::location::{Building, Location};
use crate::projector::Projector;
use crate::table::Table;
use rusqlite::{params, Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS equipment (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                type TEXT NOT NULL,
                building TEXT NOT NULL,
                floor INTEGER NOT NULL,
                room INTEGER NOT NULL,
                value INTEGER NOT NULL,
                extra_data TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn insert_equipment(&self, equipment: &Equipment) -> Result<i64> {
        let (type_name, building, floor, room, value, extra_data) = match equipment {
            Equipment::Table(t) => (
                "Table",
                t.location.building.to_code(),
                t.location.floor,
                t.location.room,
                t.value,
                t.seats.to_string(),
            ),
            Equipment::Chair(c) => (
                "Chair",
                c.location.building.to_code(),
                c.location.floor,
                c.location.room,
                c.value,
                format!("{}", c.chair_type),
            ),
            Equipment::Projector(p) => (
                "Projector",
                p.location.building.to_code(),
                p.location.floor,
                p.location.room,
                p.value,
                p.lumens.to_string(),
            ),
        };

        self.conn.execute(
            "INSERT INTO equipment (type, building, floor, room, value, extra_data) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![type_name, building, floor, room, value, extra_data],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_all_equipment(&self) -> Result<Vec<Equipment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, type, building, floor, room, value, extra_data 
             FROM equipment 
             ORDER BY building, floor, room, type",
        )?;

        let equipment_iter = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let type_name: String = row.get(1)?;
            let building_code: String = row.get(2)?;
            let floor: u8 = row.get(3)?;
            let room: u8 = row.get(4)?;
            let value: u32 = row.get(5)?;
            let extra_data: String = row.get(6)?;

            let building = Building::try_from(building_code.as_str()).unwrap();
            let location = Location::try_from((building, floor, room)).unwrap();

            let equipment = match type_name.as_str() {
                "Table" => {
                    let seats = extra_data.parse::<u8>().unwrap_or(0);
                    Equipment::Table(Table::try_from((location.clone(), value, seats)).unwrap().with_id(id))
                }
                "Chair" => {
                    let chair_type = ChairType::try_from(extra_data.as_str())
                        .unwrap_or(ChairType::Annad);
                    Equipment::Chair(Chair::try_from((location.clone(), value, chair_type)).unwrap().with_id(id))
                }
                "Projector" => {
                    let lumens = extra_data.parse::<u32>().unwrap_or(0);
                    Equipment::Projector(Projector::try_from((location.clone(), value, lumens)).unwrap().with_id(id))
                }
                _ => return Err(rusqlite::Error::InvalidQuery),
            };

            Ok(equipment)
        })?;

        let mut result = Vec::new();
        for equipment in equipment_iter {
            result.push(equipment?);
        }

        Ok(result)
    }

    pub fn get_equipment_by_id(&self, id: i64) -> Result<Option<Equipment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, type, building, floor, room, value, extra_data 
             FROM equipment 
             WHERE id = ?1",
        )?;

        let mut equipment_iter = stmt.query_map([id], |row| {
            let id: i64 = row.get(0)?;
            let type_name: String = row.get(1)?;
            let building_code: String = row.get(2)?;
            let floor: u8 = row.get(3)?;
            let room: u8 = row.get(4)?;
            let value: u32 = row.get(5)?;
            let extra_data: String = row.get(6)?;

            let building = Building::try_from(building_code.as_str()).unwrap();
            let location = Location::try_from((building, floor, room)).unwrap();

            let equipment = match type_name.as_str() {
                "Table" => {
                    let seats = extra_data.parse::<u8>().unwrap_or(0);
                    Equipment::Table(Table::try_from((location.clone(), value, seats)).unwrap().with_id(id))
                }
                "Chair" => {
                    let chair_type = ChairType::try_from(extra_data.as_str())
                        .unwrap_or(ChairType::Annad);
                    Equipment::Chair(Chair::try_from((location.clone(), value, chair_type)).unwrap().with_id(id))
                }
                "Projector" => {
                    let lumens = extra_data.parse::<u32>().unwrap_or(0);
                    Equipment::Projector(Projector::try_from((location.clone(), value, lumens)).unwrap().with_id(id))
                }
                _ => return Err(rusqlite::Error::InvalidQuery),
            };

            Ok(equipment)
        })?;

        if let Some(equipment) = equipment_iter.next() {
            Ok(Some(equipment?))
        } else {
            Ok(None)
        }
    }

    pub fn update_location(&self, id: i64, location: &Location) -> Result<()> {
        self.conn.execute(
            "UPDATE equipment SET building = ?1, floor = ?2, room = ?3 WHERE id = ?4",
            params![location.building.to_code(), location.floor, location.room, id],
        )?;
        Ok(())
    }

    pub fn delete_equipment(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM equipment WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_equipment_by_building(&self, building: Building) -> Result<Vec<Equipment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, type, building, floor, room, value, extra_data 
             FROM equipment 
             WHERE building = ?1
             ORDER BY floor, room, type",
        )?;

        let equipment_iter = stmt.query_map([building.to_code()], |row| {
            let id: i64 = row.get(0)?;
            let type_name: String = row.get(1)?;
            let building_code: String = row.get(2)?;
            let floor: u8 = row.get(3)?;
            let room: u8 = row.get(4)?;
            let value: u32 = row.get(5)?;
            let extra_data: String = row.get(6)?;

            let building = Building::try_from(building_code.as_str()).unwrap();
            let location = Location::try_from((building, floor, room)).unwrap();

            let equipment = match type_name.as_str() {
                "Table" => {
                    let seats = extra_data.parse::<u8>().unwrap_or(0);
                    Equipment::Table(Table::try_from((location.clone(), value, seats)).unwrap().with_id(id))
                }
                "Chair" => {
                    let chair_type = ChairType::try_from(extra_data.as_str())
                        .unwrap_or(ChairType::Annad);
                    Equipment::Chair(Chair::try_from((location.clone(), value, chair_type)).unwrap().with_id(id))
                }
                "Projector" => {
                    let lumens = extra_data.parse::<u32>().unwrap_or(0);
                    Equipment::Projector(Projector::try_from((location.clone(), value, lumens)).unwrap().with_id(id))
                }
                _ => return Err(rusqlite::Error::InvalidQuery),
            };

            Ok(equipment)
        })?;

        let mut result = Vec::new();
        for equipment in equipment_iter {
            result.push(equipment?);
        }

        Ok(result)
    }

    pub fn get_equipment_by_type(&self, type_name: &str) -> Result<Vec<Equipment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, type, building, floor, room, value, extra_data 
             FROM equipment 
             WHERE type = ?1
             ORDER BY building, floor, room",
        )?;

        let equipment_iter = stmt.query_map([type_name], |row| {
            let id: i64 = row.get(0)?;
            let type_name: String = row.get(1)?;
            let building_code: String = row.get(2)?;
            let floor: u8 = row.get(3)?;
            let room: u8 = row.get(4)?;
            let value: u32 = row.get(5)?;
            let extra_data: String = row.get(6)?;

            let building = Building::try_from(building_code.as_str()).unwrap();
            let location = Location::try_from((building, floor, room)).unwrap();

            let equipment = match type_name.as_str() {
                "Table" => {
                    let seats = extra_data.parse::<u8>().unwrap_or(0);
                    Equipment::Table(Table::try_from((location.clone(), value, seats)).unwrap().with_id(id))
                }
                "Chair" => {
                    let chair_type = ChairType::try_from(extra_data.as_str())
                        .unwrap_or(ChairType::Annad);
                    Equipment::Chair(Chair::try_from((location.clone(), value, chair_type)).unwrap().with_id(id))
                }
                "Projector" => {
                    let lumens = extra_data.parse::<u32>().unwrap_or(0);
                    Equipment::Projector(Projector::try_from((location.clone(), value, lumens)).unwrap().with_id(id))
                }
                _ => return Err(rusqlite::Error::InvalidQuery),
            };

            Ok(equipment)
        })?;

        let mut result = Vec::new();
        for equipment in equipment_iter {
            result.push(equipment?);
        }

        Ok(result)
    }

    pub fn get_equipment_by_room(&self, building: Building, floor: u8, room: u8) -> Result<Vec<Equipment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, type, building, floor, room, value, extra_data 
             FROM equipment 
             WHERE building = ?1 AND floor = ?2 AND room = ?3
             ORDER BY type",
        )?;

        let equipment_iter = stmt.query_map(params![building.to_code(), floor, room], |row| {
            let id: i64 = row.get(0)?;
            let type_name: String = row.get(1)?;
            let building_code: String = row.get(2)?;
            let floor: u8 = row.get(3)?;
            let room: u8 = row.get(4)?;
            let value: u32 = row.get(5)?;
            let extra_data: String = row.get(6)?;

            let building = Building::from_code(&building_code).unwrap();
            let location = Location::new(building, floor, room);

            let equipment = match type_name.as_str() {
                "Table" => {
                    let seats = extra_data.parse::<u8>().unwrap_or(0);
                    Equipment::Table(Table::try_from((location.clone(), value, seats)).unwrap().with_id(id))
                }
                "Chair" => {
                    let chair_type = ChairType::try_from(extra_data.as_str())
                        .unwrap_or(ChairType::Annad);
                    Equipment::Chair(Chair::try_from((location.clone(), value, chair_type)).unwrap().with_id(id))
                }
                "Projector" => {
                    let lumens = extra_data.parse::<u32>().unwrap_or(0);
                    Equipment::Projector(Projector::try_from((location.clone(), value, lumens)).unwrap().with_id(id))
                }
                _ => return Err(rusqlite::Error::InvalidQuery),
            };

            Ok(equipment)
        })?;

        let mut result = Vec::new();
        for equipment in equipment_iter {
            result.push(equipment?);
        }

        Ok(result)
    }

    pub fn get_equipment_by_floor(&self, building: Building, floor: u8) -> Result<Vec<Equipment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, type, building, floor, room, value, extra_data 
             FROM equipment 
             WHERE building = ?1 AND floor = ?2
             ORDER BY room, type",
        )?;

        let equipment_iter = stmt.query_map(params![building.to_code(), floor], |row| {
            let id: i64 = row.get(0)?;
            let type_name: String = row.get(1)?;
            let building_code: String = row.get(2)?;
            let floor: u8 = row.get(3)?;
            let room: u8 = row.get(4)?;
            let value: u32 = row.get(5)?;
            let extra_data: String = row.get(6)?;

            let building = Building::from_code(&building_code).unwrap();
            let location = Location::new(building, floor, room);

            let equipment = match type_name.as_str() {
                "Table" => {
                    let seats = extra_data.parse::<u8>().unwrap_or(0);
                    Equipment::Table(Table::try_from((location.clone(), value, seats)).unwrap().with_id(id))
                }
                "Chair" => {
                    let chair_type = ChairType::try_from(extra_data.as_str())
                        .unwrap_or(ChairType::Annad);
                    Equipment::Chair(Chair::try_from((location.clone(), value, chair_type)).unwrap().with_id(id))
                }
                "Projector" => {
                    let lumens = extra_data.parse::<u32>().unwrap_or(0);
                    Equipment::Projector(Projector::try_from((location.clone(), value, lumens)).unwrap().with_id(id))
                }
                _ => return Err(rusqlite::Error::InvalidQuery),
            };

            Ok(equipment)
        })?;

        let mut result = Vec::new();
        for equipment in equipment_iter {
            result.push(equipment?);
        }

        Ok(result)
    }
}
