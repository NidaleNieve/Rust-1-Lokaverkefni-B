mod chair;
mod chair_type;
mod database;
mod equipment;
mod location;
mod projector;
mod table;

use chair::Chair;
use chair_type::ChairType;
use database::Database;
use eframe::egui;
use equipment::Equipment;
use location::{Building, Location};
use projector::Projector;
use table::Table;
use std::sync::Arc;
use std::sync::Mutex;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1200.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Búnaðarlisti Tækniskólans",
        options,
        Box::new(|_cc| Ok(Box::new(EquipmentApp::new()))),
    )
}

#[derive(PartialEq)]
enum AppSection {
    Register,
    Update,
    Delete,
    Display,
}

#[derive(PartialEq, Clone, Copy)]
enum EquipmentType {
    Table,
    Chair,
    Projector,
}

#[derive(PartialEq)]
enum DisplayFilter {
    All,
    ByBuilding,
    ByType,
    ByRoom,
    ByFloor,
}

#[derive(PartialEq, Clone, Copy)]
enum SortColumn {
    Id,
    Type,
    Location,
    Value,
}

#[derive(PartialEq, Clone, Copy)]
enum SortOrder {
    Ascending,
    Descending,
}

struct EquipmentApp {
    db: Arc<Mutex<Database>>,
    current_section: AppSection,
    
    // Registration fields
    reg_equipment_type: EquipmentType,
    reg_building: Building,
    reg_floor: u8,
    reg_room: u8,
    reg_value: String,
    reg_table_seats: u8,
    reg_chair_type: ChairType,
    reg_projector_lumens: String,
    
    // Update fields
    upd_id: String,
    upd_building: Building,
    upd_floor: u8,
    upd_room: u8,
    
    // Delete fields
    del_id: String,
    
    // Display fields
    display_filter: DisplayFilter,
    display_building: Building,
    display_type: EquipmentType,
    display_room_floor: u8,
    display_room_number: u8,
    display_floor: u8,
    
    // Display results
    displayed_equipment: Vec<Equipment>,
    display_output: String,
    
    // Sorting
    sort_column: Option<SortColumn>,
    sort_order: SortOrder,
    
    // Messages
    message: String,
    error_message: String,
    
    // Search
    search_query: String,
    
    // Statistics
    show_stats: bool,
}

impl EquipmentApp {
    fn new() -> Self {
        let db = Database::new("equipment.db").expect("Failed to create database");
        
        Self {
            db: Arc::new(Mutex::new(db)),
            current_section: AppSection::Register,
            reg_equipment_type: EquipmentType::Table,
            reg_building: Building::Hafnarfjordur,
            reg_floor: 1,
            reg_room: 1,
            reg_value: String::new(),
            reg_table_seats: 4,
            reg_chair_type: ChairType::Skolastoll,
            reg_projector_lumens: String::new(),
            upd_id: String::new(),
            upd_building: Building::Hafnarfjordur,
            upd_floor: 1,
            upd_room: 1,
            del_id: String::new(),
            display_filter: DisplayFilter::All,
            display_building: Building::Hafnarfjordur,
            display_type: EquipmentType::Table,
            display_room_floor: 1,
            display_room_number: 1,
            display_floor: 1,
            displayed_equipment: Vec::new(),
            display_output: String::new(),
            sort_column: None,
            sort_order: SortOrder::Ascending,
            message: String::new(),
            error_message: String::new(),
            search_query: String::new(),
            show_stats: false,
        }
    }
    
    fn render_location_input(
        ui: &mut egui::Ui,
        building: &mut Building,
        floor: &mut u8,
        room: &mut u8,
    ) {
        ui.horizontal(|ui| {
            ui.label("Hús:");
            egui::ComboBox::from_id_source("building_combo")
                .selected_text(format!("{}", building))
                .show_ui(ui, |ui| {
                    for b in Building::all() {
                        ui.selectable_value(building, b, format!("{}", b));
                    }
                });
        });
        
        ui.horizontal(|ui| {
            ui.label("Hæð:");
            ui.add(egui::Slider::new(floor, 0..=9).text("hæð"));
        });
        
        ui.horizontal(|ui| {
            ui.label("Herbergi:");
            ui.add(egui::Slider::new(room, 0..=99).text("herbergi"));
        });
        
        ui.label(format!(
            "Staðsetning: {}-{}{}",
            building.to_code(),
            floor,
            room
        ));
    }
    
    fn register_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("📝 Skrá nýjan búnað");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("Tegund búnaðar:");
            ui.radio_value(&mut self.reg_equipment_type, EquipmentType::Table, "🪑 Borð");
            ui.radio_value(&mut self.reg_equipment_type, EquipmentType::Chair, "💺 Stóll");
            ui.radio_value(&mut self.reg_equipment_type, EquipmentType::Projector, "📽️ Skjávarpi");
        });
        
        ui.add_space(10.0);
        
        Self::render_location_input(
            ui,
            &mut self.reg_building,
            &mut self.reg_floor,
            &mut self.reg_room,
        );
        
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            ui.label("Verðmæti (kr):");
            ui.text_edit_singleline(&mut self.reg_value);
        });
        
        match self.reg_equipment_type {
            EquipmentType::Table => {
                ui.horizontal(|ui| {
                    ui.label("Fjöldi sæta:");
                    ui.add(egui::Slider::new(&mut self.reg_table_seats, 1..=20).text("sæti"));
                });
            }
            EquipmentType::Chair => {
                ui.horizontal(|ui| {
                    ui.label("Tegund stóls:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.reg_chair_type))
                        .show_ui(ui, |ui| {
                            for ct in ChairType::all() {
                                ui.selectable_value(&mut self.reg_chair_type, ct, format!("{}", ct));
                            }
                        });
                });
            }
            EquipmentType::Projector => {
                ui.horizontal(|ui| {
                    ui.label("Lumens:");
                    ui.text_edit_singleline(&mut self.reg_projector_lumens);
                });
            }
        }
        
        ui.add_space(20.0);
        
        if ui.button("✅ Skrá búnað").clicked() {
            self.register_equipment();
        }
    }
    
    fn register_equipment(&mut self) {
        self.error_message.clear();
        self.message.clear();
        
        let value = match self.reg_value.parse::<u32>() {
            Ok(v) => v,
            Err(_) => {
                self.error_message = "Verðmæti verður að vera tala".to_string();
                return;
            }
        };
        
        let location = Location::new(self.reg_building, self.reg_floor, self.reg_room);
        
        let equipment = match self.reg_equipment_type {
            EquipmentType::Table => {
                Equipment::Table(Table::new(location, value, self.reg_table_seats))
            }
            EquipmentType::Chair => {
                Equipment::Chair(Chair::new(location, value, self.reg_chair_type))
            }
            EquipmentType::Projector => {
                let lumens = match self.reg_projector_lumens.parse::<u32>() {
                    Ok(l) => l,
                    Err(_) => {
                        self.error_message = "Lumens verður að vera tala".to_string();
                        return;
                    }
                };
                Equipment::Projector(Projector::new(location, value, lumens))
            }
        };
        
        let db = self.db.lock().unwrap();
        match db.insert_equipment(&equipment) {
            Ok(id) => {
                self.message = format!("✅ Búnaður skráður með ID: {}", id);
                self.reg_value.clear();
                self.reg_projector_lumens.clear();
            }
            Err(e) => {
                self.error_message = format!("❌ Villa við skráningu: {}", e);
            }
        }
    }
    
    fn update_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔄 Uppfæra staðsetningu");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("ID búnaðar:");
            ui.text_edit_singleline(&mut self.upd_id);
        });
        
        ui.add_space(10.0);
        
        ui.label("Ný staðsetning:");
        Self::render_location_input(
            ui,
            &mut self.upd_building,
            &mut self.upd_floor,
            &mut self.upd_room,
        );
        
        ui.add_space(20.0);
        
        if ui.button("✅ Uppfæra staðsetningu").clicked() {
            self.update_location();
        }
    }
    
    fn update_location(&mut self) {
        self.error_message.clear();
        self.message.clear();
        
        let id = match self.upd_id.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                self.error_message = "ID verður að vera tala".to_string();
                return;
            }
        };
        
        let location = Location::new(self.upd_building, self.upd_floor, self.upd_room);
        
        let db = self.db.lock().unwrap();
        match db.update_location(id, &location) {
            Ok(_) => {
                self.message = format!("✅ Staðsetning uppfærð fyrir búnað með ID: {}", id);
                self.upd_id.clear();
            }
            Err(e) => {
                self.error_message = format!("❌ Villa við uppfærslu: {}", e);
            }
        }
    }
    
    fn delete_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗑️ Eyða búnaði");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("ID búnaðar:");
            ui.text_edit_singleline(&mut self.del_id);
        });
        
        ui.add_space(20.0);
        
        if ui.button("❌ Eyða búnaði").clicked() {
            self.delete_equipment();
        }
    }
    
    fn delete_equipment(&mut self) {
        self.error_message.clear();
        self.message.clear();
        
        let id = match self.del_id.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                self.error_message = "ID verður að vera tala".to_string();
                return;
            }
        };
        
        let db = self.db.lock().unwrap();
        match db.delete_equipment(id) {
            Ok(_) => {
                self.message = format!("✅ Búnaði með ID {} eytt", id);
                self.del_id.clear();
            }
            Err(e) => {
                self.error_message = format!("❌ Villa við eyðingu: {}", e);
            }
        }
    }
    
    fn display_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("📋 Birta búnað");
        ui.separator();
        
        // Statistics toggle
        ui.horizontal(|ui| {
            ui.label("📊");
            if ui.button(if self.show_stats { "Fela tölfræði" } else { "Sýna tölfræði" }).clicked() {
                self.show_stats = !self.show_stats;
                if self.show_stats && self.displayed_equipment.is_empty() {
                    // Load all equipment for stats
                    let db = self.db.lock().unwrap();
                    if let Ok(eq) = db.get_all_equipment() {
                        self.displayed_equipment = eq;
                    }
                }
            }
        });
        
        // Show statistics if enabled
        if self.show_stats {
            ui.add_space(10.0);
            self.show_statistics(ui);
            ui.add_space(10.0);
            ui.separator();
        }
        
        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍 Leita:");
            ui.text_edit_singleline(&mut self.search_query);
        });
        
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            ui.label("Sía:");
            ui.radio_value(&mut self.display_filter, DisplayFilter::All, "Allur búnaður");
            ui.radio_value(&mut self.display_filter, DisplayFilter::ByBuilding, "Eftir húsi");
            ui.radio_value(&mut self.display_filter, DisplayFilter::ByType, "Eftir tegund");
            ui.radio_value(&mut self.display_filter, DisplayFilter::ByRoom, "Eftir stofu");
            ui.radio_value(&mut self.display_filter, DisplayFilter::ByFloor, "Eftir hæð");
        });
        
        ui.add_space(10.0);
        
        match self.display_filter {
            DisplayFilter::All => {}
            DisplayFilter::ByBuilding => {
                ui.horizontal(|ui| {
                    ui.label("Hús:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.display_building))
                        .show_ui(ui, |ui| {
                            for b in Building::all() {
                                ui.selectable_value(&mut self.display_building, b, format!("{}", b));
                            }
                        });
                });
            }
            DisplayFilter::ByType => {
                ui.horizontal(|ui| {
                    ui.label("Tegund:");
                    ui.radio_value(&mut self.display_type, EquipmentType::Table, "Borð");
                    ui.radio_value(&mut self.display_type, EquipmentType::Chair, "Stóll");
                    ui.radio_value(&mut self.display_type, EquipmentType::Projector, "Skjávarpi");
                });
            }
            DisplayFilter::ByRoom => {
                ui.horizontal(|ui| {
                    ui.label("Hús:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.display_building))
                        .show_ui(ui, |ui| {
                            for b in Building::all() {
                                ui.selectable_value(&mut self.display_building, b, format!("{}", b));
                            }
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Hæð:");
                    ui.add(egui::Slider::new(&mut self.display_room_floor, 0..=9).text("hæð"));
                });
                ui.horizontal(|ui| {
                    ui.label("Herbergi:");
                    ui.add(egui::Slider::new(&mut self.display_room_number, 0..=99).text("herbergi"));
                });
            }
            DisplayFilter::ByFloor => {
                ui.horizontal(|ui| {
                    ui.label("Hús:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.display_building))
                        .show_ui(ui, |ui| {
                            for b in Building::all() {
                                ui.selectable_value(&mut self.display_building, b, format!("{}", b));
                            }
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Hæð:");
                    ui.add(egui::Slider::new(&mut self.display_floor, 0..=9).text("hæð"));
                });
            }
        }
        
        ui.add_space(10.0);
        
        if ui.button("🔍 Birta").clicked() {
            self.load_equipment();
        }
        
        if ui.button("💾 Vista í JSON").clicked() {
            self.save_to_json();
        }
        
        if ui.button("📂 Hlaða úr JSON").clicked() {
            self.load_from_json();
        }
        
        ui.add_space(10.0);
        
        // Display table with sortable columns
        if !self.displayed_equipment.is_empty() {
            ui.separator();
            ui.label(format!("Fjöldi: {} atriði", self.displayed_equipment.len()));
            
            // Sort reset button
            if self.sort_column.is_some() {
                if ui.button("🔄 Endurstilla röðun").clicked() {
                    self.sort_column = None;
                    self.load_equipment();
                }
            }
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("equipment_grid")
                    .striped(true)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        // Header
                        if ui.button("ID ▲▼").clicked() {
                            self.toggle_sort(SortColumn::Id);
                        }
                        if ui.button("Tegund ▲▼").clicked() {
                            self.toggle_sort(SortColumn::Type);
                        }
                        if ui.button("Staðsetning ▲▼").clicked() {
                            self.toggle_sort(SortColumn::Location);
                        }
                        if ui.button("Verðmæti ▲▼").clicked() {
                            self.toggle_sort(SortColumn::Value);
                        }
                        ui.label("Lýsing");
                        ui.end_row();
                        
                        // Data rows
                        for equipment in &self.displayed_equipment {
                            let id = equipment.get_id().unwrap_or(0);
                            let id_str = id.to_string();
                            
                            // Filter by search
                            let equipment_str = format!("{}", equipment).to_lowercase();
                            if !self.search_query.is_empty() && !equipment_str.contains(&self.search_query.to_lowercase()) {
                                continue;
                            }
                            
                            ui.label(&id_str);
                            ui.label(equipment.get_type_name());
                            
                            let location_str = match equipment {
                                Equipment::Table(t) => format!("{}", t.location),
                                Equipment::Chair(c) => format!("{}", c.location),
                                Equipment::Projector(p) => format!("{}", p.location),
                            };
                            ui.label(location_str);
                            
                            let value = match equipment {
                                Equipment::Table(t) => t.value,
                                Equipment::Chair(c) => c.value,
                                Equipment::Projector(p) => p.value,
                            };
                            ui.label(format!("{} kr.", value));
                            
                            ui.label(format!("{}", equipment));
                            ui.end_row();
                        }
                    });
            });
        }
    }
    
    fn toggle_sort(&mut self, column: SortColumn) {
        if let Some(current_col) = self.sort_column {
            if current_col == column {
                // Toggle order
                self.sort_order = match self.sort_order {
                    SortOrder::Ascending => SortOrder::Descending,
                    SortOrder::Descending => SortOrder::Ascending,
                };
            } else {
                self.sort_column = Some(column);
                self.sort_order = SortOrder::Ascending;
            }
        } else {
            self.sort_column = Some(column);
            self.sort_order = SortOrder::Ascending;
        }
        
        self.sort_equipment();
    }
    
    fn show_statistics(&self, ui: &mut egui::Ui) {
        let db = self.db.lock().unwrap();
        let all_equipment = db.get_all_equipment().unwrap_or_default();
        
        let total_count = all_equipment.len();
        let table_count = all_equipment.iter().filter(|e| matches!(e, Equipment::Table(_))).count();
        let chair_count = all_equipment.iter().filter(|e| matches!(e, Equipment::Chair(_))).count();
        let projector_count = all_equipment.iter().filter(|e| matches!(e, Equipment::Projector(_))).count();
        
        let total_value: u32 = all_equipment.iter().map(|e| match e {
            Equipment::Table(t) => t.value,
            Equipment::Chair(c) => c.value,
            Equipment::Projector(p) => p.value,
        }).sum();
        
        // Count by building
        let ha_count = all_equipment.iter().filter(|e| {
            let loc = match e {
                Equipment::Table(t) => &t.location,
                Equipment::Chair(c) => &c.location,
                Equipment::Projector(p) => &p.location,
            };
            matches!(loc.building, Building::Hafnarfjordur)
        }).count();
        
        let h_count = all_equipment.iter().filter(|e| {
            let loc = match e {
                Equipment::Table(t) => &t.location,
                Equipment::Chair(c) => &c.location,
                Equipment::Projector(p) => &p.location,
            };
            matches!(loc.building, Building::Hateigssvegur)
        }).count();
        
        let s_count = all_equipment.iter().filter(|e| {
            let loc = match e {
                Equipment::Table(t) => &t.location,
                Equipment::Chair(c) => &c.location,
                Equipment::Projector(p) => &p.location,
            };
            matches!(loc.building, Building::Skolavorduhollt)
        }).count();
        
        ui.group(|ui| {
            ui.heading("📊 Tölfræði");
            ui.add_space(5.0);
            
            egui::Grid::new("stats_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("📦 Heildarfjöldi búnaðar:");
                    ui.label(format!("{}", total_count));
                    ui.end_row();
                    
                    ui.label("💰 Heildarverðmæti:");
                    ui.label(format!("{} kr.", total_value));
                    ui.end_row();
                    
                    ui.separator();
                    ui.separator();
                    ui.end_row();
                    
                    ui.label("🪑 Borð:");
                    ui.label(format!("{} ({:.1}%)", table_count, (table_count as f32 / total_count as f32) * 100.0));
                    ui.end_row();
                    
                    ui.label("💺 Stólar:");
                    ui.label(format!("{} ({:.1}%)", chair_count, (chair_count as f32 / total_count as f32) * 100.0));
                    ui.end_row();
                    
                    ui.label("📽️ Skjávarpar:");
                    ui.label(format!("{} ({:.1}%)", projector_count, (projector_count as f32 / total_count as f32) * 100.0));
                    ui.end_row();
                    
                    ui.separator();
                    ui.separator();
                    ui.end_row();
                    
                    ui.label("🏢 Hafnarfjörður:");
                    ui.label(format!("{} ({:.1}%)", ha_count, (ha_count as f32 / total_count as f32) * 100.0));
                    ui.end_row();
                    
                    ui.label("🏢 Háteigsvegur:");
                    ui.label(format!("{} ({:.1}%)", h_count, (h_count as f32 / total_count as f32) * 100.0));
                    ui.end_row();
                    
                    ui.label("🏢 Skólavörðuholt:");
                    ui.label(format!("{} ({:.1}%)", s_count, (s_count as f32 / total_count as f32) * 100.0));
                    ui.end_row();
                });
        });
    }
    
    fn sort_equipment(&mut self) {
        if let Some(column) = self.sort_column {
            let ascending = matches!(self.sort_order, SortOrder::Ascending);
            
            self.displayed_equipment.sort_by(|a, b| {
                let cmp = match column {
                    SortColumn::Id => {
                        a.get_id().unwrap_or(0).cmp(&b.get_id().unwrap_or(0))
                    }
                    SortColumn::Type => {
                        a.get_type_name().cmp(b.get_type_name())
                    }
                    SortColumn::Location => {
                        let loc_a = match a {
                            Equipment::Table(t) => &t.location,
                            Equipment::Chair(c) => &c.location,
                            Equipment::Projector(p) => &p.location,
                        };
                        let loc_b = match b {
                            Equipment::Table(t) => &t.location,
                            Equipment::Chair(c) => &c.location,
                            Equipment::Projector(p) => &p.location,
                        };
                        
                        format!("{}", loc_a).cmp(&format!("{}", loc_b))
                    }
                    SortColumn::Value => {
                        let val_a = match a {
                            Equipment::Table(t) => t.value,
                            Equipment::Chair(c) => c.value,
                            Equipment::Projector(p) => p.value,
                        };
                        let val_b = match b {
                            Equipment::Table(t) => t.value,
                            Equipment::Chair(c) => c.value,
                            Equipment::Projector(p) => p.value,
                        };
                        val_a.cmp(&val_b)
                    }
                };
                
                if ascending { cmp } else { cmp.reverse() }
            });
        }
    }
    
    fn load_equipment(&mut self) {
        self.error_message.clear();
        
        let equipment = {
            let db = self.db.lock().unwrap();
            let result = match self.display_filter {
                DisplayFilter::All => db.get_all_equipment(),
                DisplayFilter::ByBuilding => db.get_equipment_by_building(self.display_building),
                DisplayFilter::ByType => {
                    let type_name = match self.display_type {
                        EquipmentType::Table => "Table",
                        EquipmentType::Chair => "Chair",
                        EquipmentType::Projector => "Projector",
                    };
                    db.get_equipment_by_type(type_name)
                }
                DisplayFilter::ByRoom => db.get_equipment_by_room(
                    self.display_building,
                    self.display_room_floor,
                    self.display_room_number,
                ),
                DisplayFilter::ByFloor => {
                    db.get_equipment_by_floor(self.display_building, self.display_floor)
                }
            };
            result
        };
        
        match equipment {
            Ok(equipment) => {
                self.displayed_equipment = equipment;
                self.sort_equipment();
                
                let mut output = String::new();
                for eq in &self.displayed_equipment {
                    output.push_str(&format!("{}\n", eq));
                }
                self.display_output = output;
            }
            Err(e) => {
                self.error_message = format!("❌ Villa við birtingu: {}", e);
            }
        }
    }
    
    fn save_to_json(&mut self) {
        self.error_message.clear();
        self.message.clear();
        
        let db = self.db.lock().unwrap();
        match db.get_all_equipment() {
            Ok(equipment) => {
                let json = serde_json::to_string_pretty(&equipment).unwrap();
                match std::fs::write("equipment.json", json) {
                    Ok(_) => {
                        self.message = "✅ Gögn vistuð í equipment.json".to_string();
                    }
                    Err(e) => {
                        self.error_message = format!("❌ Villa við vistun: {}", e);
                    }
                }
            }
            Err(e) => {
                self.error_message = format!("❌ Villa við lestur úr gagnagrunni: {}", e);
            }
        }
    }
    
    fn load_from_json(&mut self) {
        self.error_message.clear();
        self.message.clear();
        
        match std::fs::read_to_string("equipment.json") {
            Ok(json) => {
                match serde_json::from_str::<Vec<Equipment>>(&json) {
                    Ok(equipment) => {
                        let db = self.db.lock().unwrap();
                        let mut count = 0;
                        for mut eq in equipment {
                            eq.set_id(0); // Reset ID for auto-increment
                            if db.insert_equipment(&eq).is_ok() {
                                count += 1;
                            }
                        }
                        self.message = format!("✅ {} búnaður hlaðinn úr JSON", count);
                    }
                    Err(e) => {
                        self.error_message = format!("❌ Villa við að lesa JSON: {}", e);
                    }
                }
            }
            Err(e) => {
                self.error_message = format!("❌ Villa við að opna skrá: {}", e);
            }
        }
    }
}

impl eframe::App for EquipmentApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Custom style
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
        ctx.set_style(style);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header with gradient-like effect
            ui.horizontal(|ui| {
                ui.heading("🏫 Búnaðarlisti Tækniskólans");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Gagnagrunnur: equipment.db"));
                });
            });
            ui.separator();
            
            // Navigation with better styling
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing.x = 4.0;
                
                let register_btn = ui.selectable_label(
                    self.current_section == AppSection::Register,
                    "📝 Skrá"
                );
                if register_btn.clicked() {
                    self.current_section = AppSection::Register;
                }
                
                let update_btn = ui.selectable_label(
                    self.current_section == AppSection::Update,
                    "🔄 Uppfæra"
                );
                if update_btn.clicked() {
                    self.current_section = AppSection::Update;
                }
                
                let delete_btn = ui.selectable_label(
                    self.current_section == AppSection::Delete,
                    "🗑️ Eyða"
                );
                if delete_btn.clicked() {
                    self.current_section = AppSection::Delete;
                }
                
                let display_btn = ui.selectable_label(
                    self.current_section == AppSection::Display,
                    "📋 Birta"
                );
                if display_btn.clicked() {
                    self.current_section = AppSection::Display;
                }
            });
            
            ui.separator();
            
            // Messages with better visibility
            if !self.message.is_empty() {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("✓").color(egui::Color32::GREEN).size(20.0));
                    ui.label(egui::RichText::new(&self.message).color(egui::Color32::GREEN).strong());
                });
                ui.add_space(5.0);
            }
            if !self.error_message.is_empty() {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("✗").color(egui::Color32::RED).size(20.0));
                    ui.label(egui::RichText::new(&self.error_message).color(egui::Color32::RED).strong());
                });
                ui.add_space(5.0);
            }
            
            ui.add_space(10.0);
            
            // Content
            match self.current_section {
                AppSection::Register => self.register_section(ui),
                AppSection::Update => self.update_section(ui),
                AppSection::Delete => self.delete_section(ui),
                AppSection::Display => self.display_section(ui),
            }
        });
    }
}
