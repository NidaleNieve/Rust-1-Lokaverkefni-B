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
use eframe::egui::{IconData, TextureHandle};
use eframe::epaint::ColorImage;
use rfd::FileDialog;
use equipment::Equipment;
use location::{Building, Location};
use projector::Projector;
use table::Table;
use std::sync::Arc;
use std::sync::Mutex;
use std::process::Command;
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    // Build viewport and set app icon if available
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1400.0, 900.0])
        .with_min_inner_size([1200.0, 700.0]);
    if let Ok(bytes) = std::fs::read("assets/app_icon.jpg") {
        if let Ok(img) = image::load_from_memory(&bytes) {
            let rgba = img.to_rgba8();
            let (width, height) = (rgba.width(), rgba.height());
            let icon = IconData { rgba: rgba.to_vec(), width, height };
            viewport = viewport.with_icon(icon);
        }
    }
    let options = eframe::NativeOptions { viewport, ..Default::default() };

    eframe::run_native(
        "Búnaðarlisti Tækniskólans",
        options,
        Box::new(|_cc| Ok(Box::new(EquipmentApp::new()))),
    )
}

#[derive(PartialEq)]
enum AppSection {
    Register,
    Edit,
    Search,
    Print,
}

#[derive(PartialEq, Clone, Copy)]
enum EquipmentType {
    Table,
    Chair,
    Projector,
}

#[derive(PartialEq, Copy, Clone)]
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
    // In-app icon texture
    app_icon_tex: Option<TextureHandle>,
    // Navigation context
    came_from_search: bool,
    came_from_print: bool,
    
    // Registration fields
    reg_equipment_type: EquipmentType,
    reg_building: Building,
    reg_floor: u8,
    reg_room: u8,
    reg_value: String,
    reg_table_seats: u8,
    reg_chair_type: ChairType,
    reg_projector_lumens: String,
    
    // Edit fields (combined update/delete)
    edit_id: String,
    // Debounce timestamp for auto-fetching by ID in Edit
    edit_id_changed_at: Option<std::time::Instant>,
    edit_equipment: Option<Equipment>,
    edit_building: Building,
    edit_floor: u8,
    edit_room: u8,
    
    // Search fields
    search_query: String,
    search_results: Vec<Equipment>,
    search_selected_index: Option<usize>,
    search_initialized: bool,
    
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
    
    // Statistics
    show_stats: bool,
    // Sidebar toggle
    show_sidebar: bool,
}

impl EquipmentApp {
    // Draw a radio button with black fill when selected, preserving label color
    fn radio_black_value<T: PartialEq + Copy>(ui: &mut egui::Ui, value: &mut T, selected: T, label: &str) {
        ui.horizontal(|ui| {
            let size = ui.spacing().interact_size.y;
            let (rect, mut resp) = ui.allocate_at_least(egui::vec2(size, size), egui::Sense::click());
            let is_selected = *value == selected;
            if resp.clicked() { *value = selected; }
            // Paint black-dot radio
            let center = rect.center();
            let outer_r = rect.height() * 0.35;
            let inner_r = outer_r * 0.55;
            ui.painter().circle_stroke(center, outer_r, egui::Stroke { width: 1.5, color: egui::Color32::BLACK });
            if is_selected {
                ui.painter().circle_filled(center, inner_r, egui::Color32::BLACK);
            }
            ui.add_space(6.0);
            let label_resp = ui.add(egui::Label::new(label).sense(egui::Sense::click()));
            if label_resp.clicked() { *value = selected; }
        });
    }
    fn new() -> Self {
        let db = Database::new("equipment.db").expect("Failed to create database");
        
        let mut this = Self {
            db: Arc::new(Mutex::new(db)),
            current_section: AppSection::Register,
            app_icon_tex: None,
            came_from_search: false,
            came_from_print: false,
            reg_equipment_type: EquipmentType::Table,
            reg_building: Building::Hafnarfjordur,
            reg_floor: 1,
            reg_room: 1,
            reg_value: String::new(),
            reg_table_seats: 4,
            reg_chair_type: ChairType::Skolastoll,
            reg_projector_lumens: String::new(),
            edit_id: String::new(),
            edit_id_changed_at: None,
            edit_equipment: None,
            edit_building: Building::Hafnarfjordur,
            edit_floor: 1,
            edit_room: 1,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected_index: None,
            search_initialized: false,
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
            show_stats: false,
            show_sidebar: false,
        };
        // Run initial search so users don't need to click "Sækja" or type to see data
        this.perform_search();
        this.search_initialized = true;
        this
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
        
        // Custom black-dot radios (button only) with extra spacing; label stays default color
        ui.horizontal(|ui| {
            ui.label("Tegund búnaðar:");
            Self::radio_black_value(ui, &mut self.reg_equipment_type, EquipmentType::Table, "■ Borð");
            ui.add_space(12.0);
            Self::radio_black_value(ui, &mut self.reg_equipment_type, EquipmentType::Chair, "💺 Stóll");
            ui.add_space(12.0);
            Self::radio_black_value(ui, &mut self.reg_equipment_type, EquipmentType::Projector, "📽 Skjávarpi");
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
        
        let location = match Location::try_from((self.reg_building, self.reg_floor, self.reg_room)) {
            Ok(loc) => loc,
            Err(e) => { self.error_message = e; return; }
        };
        
        let equipment = match self.reg_equipment_type {
            EquipmentType::Table => {
                match Table::try_from((location, value, self.reg_table_seats)) {
                    Ok(t) => Equipment::Table(t),
                    Err(e) => { self.error_message = e; return; }
                }
            }
            EquipmentType::Chair => {
                match Chair::try_from((location, value, self.reg_chair_type)) {
                    Ok(c) => Equipment::Chair(c),
                    Err(e) => { self.error_message = e; return; }
                }
            }
            EquipmentType::Projector => {
                let lumens = match self.reg_projector_lumens.parse::<u32>() {
                    Ok(l) => l,
                    Err(_) => { self.error_message = "Lumens verður að vera tala".to_string(); return; }
                };
                match Projector::try_from((location, value, lumens)) {
                    Ok(p) => Equipment::Projector(p),
                    Err(e) => { self.error_message = e; return; }
                }
            }
        };
        
        let db = self.db.lock().unwrap();
        match db.insert_equipment(&equipment) {
            Ok(id) => {
                self.message = format!(" Búnaður skráður með ID: {}", id);
                self.reg_value.clear();
                self.reg_projector_lumens.clear();
            }
            Err(e) => {
                self.error_message = format!(" Villa við skráningu: {}", e);
            }
        }
    }
    
    fn edit_section(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.came_from_search {
                if ui.button("⬅ Til baka í leit").clicked() {
                    self.current_section = AppSection::Search;
                    self.came_from_search = false;
                    return;
                }
                ui.separator();
            } else if self.came_from_print {
                if ui.button("⬅ Til baka í prentun").clicked() {
                    self.current_section = AppSection::Print;
                    self.came_from_print = false;
                    return;
                }
                ui.separator();
            }
           ui.heading("✏ Breyta búnaði"); // plain text pencil
        });
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("ID búnaðar:");
            let resp = ui.text_edit_singleline(&mut self.edit_id);
            if resp.changed() {
                // Start debounce timer on every change
                self.edit_id_changed_at = Some(std::time::Instant::now());
            }
            if ui.button("🔍 Sækja").clicked() {
                self.fetch_equipment_for_edit();
                // Clear debounce state after explicit fetch
                self.edit_id_changed_at = None;
            }
        });
        // Debounced auto-fetch: 300ms after last edit, if ID is numeric
        if let Some(t0) = self.edit_id_changed_at {
            if t0.elapsed() >= Duration::from_millis(300)
                && !self.edit_id.is_empty()
                && self.edit_id.chars().all(|c| c.is_ascii_digit())
            {
                self.fetch_equipment_for_edit();
                self.edit_id_changed_at = None;
            }
        }
        
        ui.add_space(10.0);
        
        // Show equipment info if fetched
        if let Some(equipment) = &self.edit_equipment {
            ui.group(|ui| {
                ui.heading("📋 Upplýsingar um búnað");
                ui.add_space(5.0);
                
                egui::Grid::new("edit_info_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("ID:");
                        ui.label(format!("{}", equipment.get_id().unwrap_or(0)));
                        ui.end_row();
                        
                        ui.label("Tegund:");
                        ui.label(equipment.get_type_name());
                        ui.end_row();
                        
                        ui.label("Staðsetning:");
                        let location = match equipment {
                            Equipment::Table(t) => &t.location,
                            Equipment::Chair(c) => &c.location,
                            Equipment::Projector(p) => &p.location,
                        };
                        ui.label(format!("{}", location));
                        ui.end_row();
                        
                        ui.label("Verðmæti:");
                        let value = match equipment {
                            Equipment::Table(t) => t.value,
                            Equipment::Chair(c) => c.value,
                            Equipment::Projector(p) => p.value,
                        };
                        ui.label(format!("{} kr.", value));
                        ui.end_row();
                        
                        ui.label("Lýsing:");
                        ui.label(format!("{}", equipment));
                        ui.end_row();
                    });
            });
            
            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);
            
            // Update location section
            ui.heading("🔄 Uppfæra staðsetningu");
            ui.add_space(10.0);
            
            Self::render_location_input(
                ui,
                &mut self.edit_building,
                &mut self.edit_floor,
                &mut self.edit_room,
            );
            
            ui.add_space(15.0);
            
            if ui.button("✅ Uppfæra staðsetningu").clicked() {
                self.update_location();
            }
            
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(15.0);
            
            // Delete section
            ui.heading("🗑 Eyða búnaði"); // works in most mono / text fonts
            ui.add_space(10.0);
            
            ui.label("⚠ Varúð: Þessi aðgerð er óafturkræf!");
            ui.add_space(10.0);
            
            if ui.button("❌ Eyða búnaði").clicked() {
                self.delete_equipment();
            }
        } else if !self.edit_id.is_empty() {
            ui.label("Sláðu inn ID og smelltu á 'Sækja' til að skoða búnað");
        }
    }
    
    fn fetch_equipment_for_edit(&mut self) {
        self.error_message.clear();
        self.message.clear();
        
        let id = match self.edit_id.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                self.error_message = "ID verður að vera tala".to_string();
                return;
            }
        };
        
        let db = self.db.lock().unwrap();
        match db.get_equipment_by_id(id) {
            Ok(Some(equipment)) => {
                // Set the location fields to current location
                let location = match &equipment {
                    Equipment::Table(t) => &t.location,
                    Equipment::Chair(c) => &c.location,
                    Equipment::Projector(p) => &p.location,
                };
                self.edit_building = location.building;
                self.edit_floor = location.floor;
                self.edit_room = location.room;
                self.edit_equipment = Some(equipment);
            }
            Ok(None) => {
                self.error_message = format!(" Búnaður með ID {} fannst ekki", id);
                self.edit_equipment = None;
            }
            Err(e) => {
                self.error_message = format!(" Villa við að sækja búnað: {}", e);
                self.edit_equipment = None;
            }
        }
    }
    
    fn update_location(&mut self) {
        self.error_message.clear();
        self.message.clear();
        
        let id = match self.edit_id.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                self.error_message = "ID verður að vera tala".to_string();
                return;
            }
        };
        
        let location = match Location::try_from((self.edit_building, self.edit_floor, self.edit_room)) {
            Ok(loc) => loc,
            Err(e) => { self.error_message = e; return; }
        };
        
        let db = self.db.lock().unwrap();
        match db.update_location(id, &location) {
            Ok(_) => {
                self.message = format!(" Staðsetning uppfærð fyrir búnað með ID: {}", id);
                // Refresh the equipment info
                drop(db);
                self.fetch_equipment_for_edit();
            }
            Err(e) => {
                self.error_message = format!(" Villa við uppfærslu: {}", e);
            }
        }
    }
    
    fn delete_equipment(&mut self) {
        self.error_message.clear();
        self.message.clear();
        
        let id = match self.edit_id.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                self.error_message = "ID verður að vera tala".to_string();
                return;
            }
        };
        
        let db = self.db.lock().unwrap();
        match db.delete_equipment(id) {
            Ok(_) => {
                self.message = format!(" Búnaði með ID {} eytt", id);
                self.edit_id.clear();
                self.edit_equipment = None;
            }
            Err(e) => {
                self.error_message = format!(" Villa við eyðingu: {}", e);
            }
        }
    }
    
    fn search_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔍 Leita að búnaði");
        ui.separator();

        // Always refresh search like the sidebar does
        self.perform_search();

        // Live search input
        ui.horizontal(|ui| {
            ui.label("Leit:");
            let changed = ui.text_edit_singleline(&mut self.search_query).changed();
            if changed {
                self.perform_search();
            }
        });
        ui.add_space(10.0);

        // Ensure search is initialized (first time entering without typing)
        if !self.search_initialized {
            self.perform_search();
            self.search_initialized = true;
        }

        // If exactly one result, auto-select it
        if self.search_results.len() == 1 {
            self.search_selected_index = Some(0);
        } else if self.search_results.is_empty() {
            self.search_selected_index = None;
        }

        // Results table with sortable columns and clickable rows
        if !self.search_results.is_empty() {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Fjöldi niðurstaðna: {} atriði", self.search_results.len()));
                if self.sort_column.is_some() {
                    if ui.button("🔄 Endurstilla röðun").clicked() {
                        self.sort_column = None;
                        self.sort_order = SortOrder::Ascending;
                    }
                }
            });
            ui.add_space(10.0);

            // Prepare sorted copy for display so we don't mutate original ordering unnecessarily
            let mut data = self.search_results.clone();
            if let Some(column) = self.sort_column {
                let ascending = matches!(self.sort_order, SortOrder::Ascending);
                data.sort_by(|a, b| {
                    let cmp = match column {
                        SortColumn::Id => a.get_id().unwrap_or(0).cmp(&b.get_id().unwrap_or(0)),
                        SortColumn::Type => a.get_type_name().cmp(b.get_type_name()),
                        SortColumn::Location => {
                            let loc_a = match a { Equipment::Table(t) => &t.location, Equipment::Chair(c) => &c.location, Equipment::Projector(p) => &p.location };
                            let loc_b = match b { Equipment::Table(t) => &t.location, Equipment::Chair(c) => &c.location, Equipment::Projector(p) => &p.location };
                            format!("{}", loc_a).cmp(&format!("{}", loc_b))
                        }
                        SortColumn::Value => {
                            let va = match a { Equipment::Table(t) => t.value, Equipment::Chair(c) => c.value, Equipment::Projector(p) => p.value };
                            let vb = match b { Equipment::Table(t) => t.value, Equipment::Chair(c) => c.value, Equipment::Projector(p) => p.value };
                            va.cmp(&vb)
                        }
                    };
                    if ascending { cmp } else { cmp.reverse() }
                });
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Use current sorted search results
                let table_data = data.clone();
                use egui_extras::{TableBuilder, Column};
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight))
                    .column(Column::initial(80.0).resizable(true)) // ID
                    .column(Column::initial(120.0).resizable(true)) // Tegund
                    .column(Column::initial(160.0).resizable(true)) // Staðsetning
                    .column(Column::initial(120.0).resizable(true)) // Verðmæti
                    .column(Column::remainder().resizable(true)); // Lýsing

                table
                    .header(22.0, |mut header| {
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("ID{}", self.sort_indicator(SortColumn::Id))).clicked() { self.toggle_sort(SortColumn::Id); } }); });
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Tegund{}", self.sort_indicator(SortColumn::Type))).clicked() { self.toggle_sort(SortColumn::Type); } }); });
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Staðsetning{}", self.sort_indicator(SortColumn::Location))).clicked() { self.toggle_sort(SortColumn::Location); } }); });
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Verðmæti{}", self.sort_indicator(SortColumn::Value))).clicked() { self.toggle_sort(SortColumn::Value); } }); });
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { ui.label("Lýsing"); }); });
                    })
                    .body(|mut body| {
                        let row_h = 22.0;
                        for (i, equipment) in table_data.iter().enumerate() {
                            let id = equipment.get_id().unwrap_or(0);
                            let location_str = match equipment { Equipment::Table(t) => format!("{}", t.location), Equipment::Chair(c) => format!("{}", c.location), Equipment::Projector(p) => format!("{}", p.location) };
                            let value = match equipment { Equipment::Table(t) => t.value, Equipment::Chair(c) => c.value, Equipment::Projector(p) => p.value };
                            body.row(row_h, |mut row| {
                                let mut clicked_any = false;
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(id.to_string()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(equipment.get_type_name()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(location_str.clone()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(format!("{} kr.", value)).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(format!("{}", equipment)).sense(egui::Sense::click())).clicked() { clicked_any = true; } });

                                if clicked_any {
                                    self.search_selected_index = Some(i);
                                    self.edit_id = id.to_string();
                                    self.fetch_equipment_for_edit();
                                    self.came_from_search = true;
                                    self.current_section = AppSection::Edit;
                                }
                            });
                        }
                    });
            });
        } else if !self.search_query.is_empty() {
            ui.label("Engar niðurstöður fundust");
        }
    }
    
    fn perform_search(&mut self) {
        self.error_message.clear();
        let db = self.db.lock().unwrap();
        match db.get_all_equipment() {
            Ok(all_equipment) => {
                let q = self.search_query.to_lowercase();
                self.search_results = all_equipment
                    .into_iter()
                    .filter(|eq| {
                        // Search across multiple columns
                        let id_match = eq.get_id().unwrap_or(0).to_string().contains(&q);
                        let type_match = eq.get_type_name().to_lowercase().contains(&q);
                        let location_match = match eq {
                            Equipment::Table(t) => format!("{}", t.location),
                            Equipment::Chair(c) => format!("{}", c.location),
                            Equipment::Projector(p) => format!("{}", p.location),
                        }
                        .to_lowercase()
                        .contains(&q);
                        let value_match = match eq {
                            Equipment::Table(t) => t.value,
                            Equipment::Chair(c) => c.value,
                            Equipment::Projector(p) => p.value,
                        }
                        .to_string()
                        .contains(&q);
                        let desc_match = format!("{}", eq).to_lowercase().contains(&q);
                        id_match || type_match || location_match || value_match || desc_match
                    })
                    .collect();
            }
            Err(e) => {
                self.error_message = format!(" Villa við leit: {}", e);
            }
        }
    }
    
    fn print_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("📋 Prenta búnað");
        ui.separator();

        // Always reload equipment like the sidebar does so the list stays fresh
        self.load_equipment();
        
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
        
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            ui.label("Sía:");
            Self::radio_black_value(ui, &mut self.display_filter, DisplayFilter::All, "Allur búnaður");
            ui.add_space(10.0);
            Self::radio_black_value(ui, &mut self.display_filter, DisplayFilter::ByBuilding, "Eftir húsi");
            ui.add_space(10.0);
            Self::radio_black_value(ui, &mut self.display_filter, DisplayFilter::ByType, "Eftir tegund");
            ui.add_space(10.0);
            Self::radio_black_value(ui, &mut self.display_filter, DisplayFilter::ByRoom, "Eftir stofu");
            ui.add_space(10.0);
            Self::radio_black_value(ui, &mut self.display_filter, DisplayFilter::ByFloor, "Eftir hæð");
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
                    Self::radio_black_value(ui, &mut self.display_type, EquipmentType::Table, "Borð");
                    ui.add_space(10.0);
                    Self::radio_black_value(ui, &mut self.display_type, EquipmentType::Chair, "Stóll");
                    ui.add_space(10.0);
                    Self::radio_black_value(ui, &mut self.display_type, EquipmentType::Projector, "Skjávarpi");
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
        
        // Align JSON and print/PDF controls on one row; remove manual "Birta" (auto-refresh is on)
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if ui.button("💾 Vista í JSON").clicked() {
                self.save_to_json();
            }
            if ui.button("📂 Hlaða úr JSON").clicked() {
                self.load_from_json();
            }
            ui.add_space(12.0);
            if ui.button("📄 Prenta lista").clicked() {
                self.print_current_list();
            }
            if ui.button("💾 Flytja út í PDF").clicked() {
                self.export_current_list_pdf();
            }
        });
        
        ui.add_space(10.0);
        
        // Display table with sortable columns
        if !self.displayed_equipment.is_empty() {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Fjöldi: {} atriði", self.displayed_equipment.len()));
                if self.sort_column.is_some() {
                    if ui.button("🔄 Endurstilla röðun").clicked() {
                        self.sort_column = None;
                        self.load_equipment();
                    }
                }
            });
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Clone current data to allow mutable self usage in callbacks
                let data = self.displayed_equipment.clone();
                use egui_extras::{TableBuilder, Column};
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight))
                    .column(Column::initial(80.0).resizable(true)) // ID
                    .column(Column::initial(120.0).resizable(true)) // Tegund
                    .column(Column::initial(160.0).resizable(true)) // Staðsetning
                    .column(Column::initial(120.0).resizable(true)) // Verðmæti
                    .column(Column::remainder().resizable(true)); // Lýsing

                table
                    .header(22.0, |mut header| {
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("ID{}", self.sort_indicator(SortColumn::Id))).clicked() { self.toggle_sort(SortColumn::Id); } }); });
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Tegund{}", self.sort_indicator(SortColumn::Type))).clicked() { self.toggle_sort(SortColumn::Type); } }); });
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Staðsetning{}", self.sort_indicator(SortColumn::Location))).clicked() { self.toggle_sort(SortColumn::Location); } }); });
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Verðmæti{}", self.sort_indicator(SortColumn::Value))).clicked() { self.toggle_sort(SortColumn::Value); } }); });
                        header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { ui.label("Lýsing"); }); });
                    })
                    .body(|mut body| {
                        let row_h = 22.0;
                        for equipment in &data {
                            let id = equipment.get_id().unwrap_or(0);
                            let location_str = match equipment { Equipment::Table(t) => format!("{}", t.location), Equipment::Chair(c) => format!("{}", c.location), Equipment::Projector(p) => format!("{}", p.location) };
                            let value = match equipment { Equipment::Table(t) => t.value, Equipment::Chair(c) => c.value, Equipment::Projector(p) => p.value };
                            body.row(row_h, |mut row| {
                                let mut clicked_any = false;
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(id.to_string()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(equipment.get_type_name()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(location_str.clone()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(format!("{} kr.", value)).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(format!("{}", equipment)).sense(egui::Sense::click())).clicked() { clicked_any = true; } });

                                if clicked_any {
                                    // From Prenta: go to Edit with back button to printing
                                    self.edit_id = id.to_string();
                                    self.fetch_equipment_for_edit();
                                    self.came_from_search = false;
                                    self.came_from_print = true;
                                    self.current_section = AppSection::Edit;
                                }
                            });
                        }
                    });
            });
        }
    }

    fn escape_html(s: &str) -> String {
        s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    }

    fn generate_print_html(&self) -> String {
        // Build a simple, print-friendly HTML page with the current displayed_equipment
        let mut rows = String::new();
        for eq in &self.displayed_equipment {
            let id = eq.get_id().unwrap_or(0).to_string();
            let typ = eq.get_type_name().to_string();
            let (location_str, value) = match eq {
                Equipment::Table(t) => (format!("{}", t.location), t.value),
                Equipment::Chair(c) => (format!("{}", c.location), c.value),
                Equipment::Projector(p) => (format!("{}", p.location), p.value),
            };
            let desc = format!("{}", eq);
            rows.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{} kr.</td><td>{}</td></tr>",
                Self::escape_html(&id),
                Self::escape_html(&typ),
                Self::escape_html(&location_str),
                value,
                Self::escape_html(&desc)
            ));
        }

        let title = "Búnaðarlisti Tækniskólans - Prentun";
        let style = r#"
            body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; margin: 24px; }
            h1 { margin-bottom: 8px; }
            .meta { color: #555; margin-bottom: 16px; }
            table { border-collapse: collapse; width: 100%; }
            th, td { border: 1px solid #ccc; padding: 8px; text-align: left; font-size: 12px; }
            th { background: #f5f7fb; }
            @media print { body { margin: 0; } }
        "#;
        let script = r#"
            window.addEventListener('load', () => {
                // Try to auto-open print dialog
                try { window.print(); } catch (_) {}
            });
        "#;

        format!(
            "<!DOCTYPE html><html><head><meta charset='utf-8'><title>{}</title><style>{}</style><script>{}</script></head><body><h1>{}</h1><div class='meta'>Fjöldi: {} atriði</div><table><thead><tr><th>ID</th><th>Tegund</th><th>Staðsetning</th><th>Verðmæti</th><th>Lýsing</th></tr></thead><tbody>{}</tbody></table></body></html>",
            title, style, script, title, self.displayed_equipment.len(), rows
        )
    }

    fn print_current_list(&mut self) {
        // Write HTML to a temporary file and open it to trigger the OS print dialog
        self.error_message.clear();
        self.message.clear();
        let html = self.generate_print_html();
        let mut path = std::env::temp_dir();
        path.push("bunadarlisti_prenta.html");
        match std::fs::write(&path, html) {
            Ok(_) => {
                // On macOS, 'open' will use default browser which will run window.print()
                let _ = Command::new("open").arg(&path).spawn();
                self.message = "📄 Opnaði prentglugga í vafra".into();
            }
            Err(e) => {
                self.error_message = format!(" Gat ekki útbúið prentun: {}", e);
            }
        }
    }

    fn export_current_list_pdf(&mut self) {
        self.error_message.clear();
        self.message.clear();
        if let Some(dest) = FileDialog::new().set_file_name("bunadarlisti.pdf").add_filter("PDF", &["pdf"]).save_file() {
            // Generate a simple PDF with a table listing the current displayed_equipment
            use printpdf::*;
            let (doc, page1, layer1) = PdfDocument::new("Búnaðarlisti", Mm(210.0), Mm(297.0), "Layer 1");
            let mut current_layer = doc.get_page(page1).get_layer(layer1);

            // Basic text config
            let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
            let title = "Búnaðarlisti Tækniskólans";
            let mut y = Mm(280.0);
            current_layer.use_text(title, 16.0, Mm(14.0), y, &font);
            y = Mm(y.0 - 8.0);
            current_layer.use_text(format!("Fjöldi: {} atriði", self.displayed_equipment.len()), 10.0, Mm(14.0), y, &font);
            y = Mm(y.0 - 10.0);

            // Table headers
            let headers = ["ID", "Tegund", "Staðsetning", "Verðmæti", "Lýsing"];
            let col_x = [Mm(14.0), Mm(34.0), Mm(70.0), Mm(110.0), Mm(140.0)];
            for (i, h) in headers.iter().enumerate() {
                current_layer.use_text(h.to_string(), 10.0, col_x[i], y, &font);
            }
            y = Mm(y.0 - 6.0);

            // Rows (simple flow, wraps to new page if needed)
            let line_h = 5.5;
            let mut page = 1;
            for eq in &self.displayed_equipment {
                if y.0 < 20.0 {
                    // new page
                    page += 1;
                    let (p, l) = doc.add_page(Mm(210.0), Mm(297.0), format!("Layer {}", page));
                    current_layer = doc.get_page(p).get_layer(l);
                    y = Mm(280.0);
                    // re-draw headers
                    for (i, h) in headers.iter().enumerate() {
                        current_layer.use_text(h.to_string(), 10.0, col_x[i], y, &font);
                    }
                    y = Mm(y.0 - 6.0);
                }

                let id = eq.get_id().unwrap_or(0).to_string();
                let typ = eq.get_type_name().to_string();
                let (location_str, value, desc) = match eq {
                    Equipment::Table(t) => (format!("{}", t.location), t.value, format!("{}", eq)),
                    Equipment::Chair(c) => (format!("{}", c.location), c.value, format!("{}", eq)),
                    Equipment::Projector(p) => (format!("{}", p.location), p.value, format!("{}", eq)),
                };

                // Draw row text
                current_layer.use_text(id, 9.0, col_x[0], y, &font);
                current_layer.use_text(typ, 9.0, col_x[1], y, &font);
                current_layer.use_text(location_str, 9.0, col_x[2], y, &font);
                current_layer.use_text(format!("{} kr.", value), 9.0, col_x[3], y, &font);
                current_layer.use_text(desc, 9.0, col_x[4], y, &font);

                y = Mm(y.0 - line_h);
            }

            match std::fs::File::create(&dest) {
                Ok(file) => {
                    let mut buf = std::io::BufWriter::new(file);
                    match doc.save(&mut buf) {
                        Ok(_) => self.message = format!("✅ Vistað PDF í {}", dest.display()),
                        Err(e) => self.error_message = format!(" Gat ekki vistað PDF: {}", e),
                    }
                }
                Err(e) => {
                    self.error_message = format!(" Gat ekki búið til skrá: {}", e);
                }
            }
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

    fn sort_indicator(&self, column: SortColumn) -> &'static str {
        if let Some(current) = self.sort_column {
            if current == column {
                return match self.sort_order {
                    SortOrder::Ascending => " ^",
                    SortOrder::Descending => " v",
                };
            }
        }
        ""
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
                    
                    ui.label("■ Borð:");
                    ui.label(format!("{} ({:.1}%)", table_count, (table_count as f32 / total_count as f32) * 100.0));
                    ui.end_row();
                    
                    ui.label("💺 Stólar:");
                    ui.label(format!("{} ({:.1}%)", chair_count, (chair_count as f32 / total_count as f32) * 100.0));
                    ui.end_row();
                    
                    ui.label("📽 Skjávarpar");
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
                self.error_message = format!(" Villa við birtingu: {}", e);
            }
        }
    }
    
    fn save_to_json(&mut self) {
        self.error_message.clear();
        self.message.clear();
        let path = FileDialog::new().set_file_name("equipment.json").add_filter("JSON", &["json"]).save_file();
        if path.is_none() { return; }
        let path = path.unwrap();
        let db = self.db.lock().unwrap();
        match db.get_all_equipment() {
            Ok(equipment) => {
                let json = serde_json::to_string_pretty(&equipment).unwrap();
                match std::fs::write(&path, json) {
                    Ok(_) => { self.message = format!(" Gögn vistuð í {}", path.display()); }
                    Err(e) => { self.error_message = format!(" Villa við vistun: {}", e); }
                }
            }
            Err(e) => { self.error_message = format!(" Villa við lestur úr gagnagrunni: {}", e); }
        }
    }
    
    fn load_from_json(&mut self) {
        self.error_message.clear();
        self.message.clear();
        let path = FileDialog::new().add_filter("JSON", &["json"]).pick_file();
        if path.is_none() { return; }
        let path = path.unwrap();
        match std::fs::read_to_string(&path) {
            Ok(json) => match serde_json::from_str::<Vec<Equipment>>(&json) {
                Ok(mut equipment) => {
                    // If JSON contains IDs, preserve them and reset AUTOINCREMENT accordingly.
                    let max_id = equipment.iter().filter_map(|e| e.get_id()).max().unwrap_or(0);
                    let mut inserted = 0;
                    {
                        let db = self.db.lock().unwrap();
                        if let Err(e) = db.clear_all_equipment() {
                            self.error_message = format!(" Tókst ekki að tæma gagnagrunn: {}", e);
                            return;
                        }
                        for eq in equipment.drain(..) {
                            if let Some(id) = eq.get_id() {
                                let _ = db.insert_equipment_with_id(id, &eq);
                                inserted += 1;
                            } else {
                                if db.insert_equipment(&eq).is_ok() { inserted += 1; }
                            }
                        }
                        if let Err(e) = db.reset_equipment_autoincrement(max_id as i64) {
                            self.error_message = format!("⚠️ Gat ekki stillt id-runu: {}", e);
                        }
                    }

                    self.displayed_equipment.clear();
                    self.search_results.clear();
                    self.sort_column = None; // default
                    self.load_equipment();
                    self.message = format!(" {} búnaður hlaðinn úr {}", inserted, path.display());
                }
                Err(e) => { self.error_message = format!(" Villa við að lesa JSON: {}", e); }
            },
            Err(e) => { self.error_message = format!(" Villa við að opna skrá: {}", e); }
        }
    }
}

impl eframe::App for EquipmentApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Refresh UI frequently so lists stay snappy
    ctx.request_repaint_after(Duration::from_millis(50));
        // Modern light blue color scheme
        let mut style = (*ctx.style()).clone();
        
        // Set colors
        let light_blue = egui::Color32::from_rgb(173, 216, 230);  // Light blue
        let hover_blue = egui::Color32::from_rgb(135, 206, 250);  // Sky blue for hover
        let active_blue = egui::Color32::from_rgb(100, 149, 237);  // Cornflower blue for active
        let text_color = egui::Color32::from_rgb(25, 25, 60);      // Dark blue text
        
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        
        // Widget colors
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(240, 248, 255);
        style.visuals.widgets.inactive.bg_fill = light_blue;
        style.visuals.widgets.hovered.bg_fill = hover_blue;
        style.visuals.widgets.active.bg_fill = active_blue;
        
        // Selection colors
        style.visuals.selection.bg_fill = active_blue;
        style.visuals.selection.stroke = egui::Stroke::new(1.0, text_color);
        
        // Make buttons rounder and more modern
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);
        
        // Add subtle shadows/strokes on hover
        style.visuals.widgets.hovered.expansion = 2.0;
        
        ctx.set_style(style);

        // Modern header with better margins and sidebar toggle
        egui::TopBottomPanel::top("app_header")
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(60, 100, 140))
                    .inner_margin(egui::Margin::symmetric(16.0, 10.0))
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(icon) = &self.app_icon_tex {
                        let desired = egui::Vec2 { x: 28.0, y: 28.0 };
                        ui.image((icon.id(), desired));
                    }
                    ui.add_space(8.0);
                    ui.heading(egui::RichText::new("Búnaðarlisti Tækniskólans").color(egui::Color32::WHITE).size(24.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let arrow = "↔"; // U+2194 (no FE0F)
                        let btn = egui::Button::new(
                            egui::RichText::new(if self.show_sidebar { format!("{arrow} Fela lista") } else { "📋 Sjá lista".into() })
                                .color(egui::Color32::WHITE),
                        );
                        //let btn = egui::Button::new(egui::RichText::new(if self.show_sidebar { "↔︎ Fela lista" } else { "📋 Sjá lista" }).color(egui::Color32::WHITE));
                        if ui.add(btn).clicked() { 
                            self.show_sidebar = !self.show_sidebar; 
                            // Refresh sidebar data when opening
                            if self.show_sidebar {
                                self.display_filter = DisplayFilter::All;
                                self.load_equipment();
                            }
                        }
                    });
                });
            });
        
        // Lazily load app icon texture for rendering inside the app
        if self.app_icon_tex.is_none() {
            if let Ok(bytes) = std::fs::read("assets/app_icon.jpg") {
                if let Ok(img) = image::load_from_memory(&bytes) {
                    let rgba = img.to_rgba8();
                    let size = [rgba.width() as usize, rgba.height() as usize];
                    let pixels = rgba.into_raw();
                    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                    let tex = ctx.load_texture("app_icon", color_image, Default::default());
                    self.app_icon_tex = Some(tex);
                }
            }
        }

        // Right sidebar with full equipment list (auto-refreshes)
        if self.show_sidebar {
            // Always reload fresh data for sidebar
            let db = self.db.lock().unwrap();
            let sidebar_data = db.get_all_equipment().unwrap_or_default();
            drop(db);
            
            egui::SidePanel::right("right_sidebar")
                .resizable(true)
                .default_width(420.0)
                .min_width(150.0)
                .max_width(ctx.screen_rect().width() * 0.7)
                .show(ctx, |ui| {
                    ui.heading("📋 Allur búnaður");
                    ui.add_space(6.0);
                    ui.label(format!("Fjöldi: {} atriði", sidebar_data.len()));
                    
                    // Sort reset button
                    if self.sort_column.is_some() {
                        if ui.button("🔄 Endurstilla röðun").clicked() {
                            self.sort_column = None;
                        }
                    }
                    
                    ui.add_space(6.0);
                    
                    // Clone data for sorting
                    let mut sorted_data = sidebar_data.clone();
                    if let Some(column) = self.sort_column {
                        let ascending = matches!(self.sort_order, SortOrder::Ascending);
                        sorted_data.sort_by(|a, b| {
                            let cmp = match column {
                                SortColumn::Id => a.get_id().unwrap_or(0).cmp(&b.get_id().unwrap_or(0)),
                                SortColumn::Type => a.get_type_name().cmp(b.get_type_name()),
                                SortColumn::Location => {
                                    let loc_a = match a { Equipment::Table(t) => &t.location, Equipment::Chair(c) => &c.location, Equipment::Projector(p) => &p.location };
                                    let loc_b = match b { Equipment::Table(t) => &t.location, Equipment::Chair(c) => &c.location, Equipment::Projector(p) => &p.location };
                                    format!("{}", loc_a).cmp(&format!("{}", loc_b))
                                }
                                SortColumn::Value => {
                                    let val_a = match a { Equipment::Table(t) => t.value, Equipment::Chair(c) => c.value, Equipment::Projector(p) => p.value };
                                    let val_b = match b { Equipment::Table(t) => t.value, Equipment::Chair(c) => c.value, Equipment::Projector(p) => p.value };
                                    val_a.cmp(&val_b)
                                }
                            };
                            if ascending { cmp } else { cmp.reverse() }
                        });
                    }
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let show_description = ui.available_width() > 460.0;
                        use egui_extras::{TableBuilder, Column};
                        let mut table = TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .cell_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight))
                            .column(Column::initial(60.0).resizable(true))
                            .column(Column::initial(110.0).resizable(true))
                            .column(Column::initial(130.0).resizable(true))
                            .column(Column::initial(110.0).resizable(true));

                        if show_description {
                            table = table.column(Column::remainder().resizable(true));
                        }

                        table
                            .header(20.0, |mut header| {
                                header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("ID{}", self.sort_indicator(SortColumn::Id))).clicked() { self.toggle_sort(SortColumn::Id); } }); });
                                header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Tegund{}", self.sort_indicator(SortColumn::Type))).clicked() { self.toggle_sort(SortColumn::Type); } }); });
                                header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Staðsetning{}", self.sort_indicator(SortColumn::Location))).clicked() { self.toggle_sort(SortColumn::Location); } }); });
                                header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { if ui.button(format!("Verðmæti{}", self.sort_indicator(SortColumn::Value))).clicked() { self.toggle_sort(SortColumn::Value); } }); });
                                if show_description {
                                    header.col(|ui| { ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| { ui.label("Lýsing"); }); });
                                }
                            })
                            .body(|mut body| {
                                let row_h = 20.0;
                                for equipment in &sorted_data {
                                    let id = equipment.get_id().unwrap_or(0);
                                    let location_str = match equipment { Equipment::Table(t) => format!("{}", t.location), Equipment::Chair(c) => format!("{}", c.location), Equipment::Projector(p) => format!("{}", p.location) };
                                    let value = match equipment { Equipment::Table(t) => t.value, Equipment::Chair(c) => c.value, Equipment::Projector(p) => p.value };
                                    body.row(row_h, |mut row| {
                                        let mut clicked_any = false;
                                        row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(id.to_string()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                        row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(equipment.get_type_name()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                        row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(location_str.clone()).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                        row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(format!("{} kr.", value)).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                        if show_description {
                                            row.col(|ui| { if ui.add_sized([ui.available_width(), row_h], egui::Label::new(format!("{}", equipment)).sense(egui::Sense::click())).clicked() { clicked_any = true; } });
                                        }

                                        if clicked_any {
                                            // From sidebar: go to Edit without back button
                                            self.edit_id = id.to_string();
                                            self.fetch_equipment_for_edit();
                                            self.came_from_search = false;
                                            self.came_from_print = false;
                                            self.current_section = AppSection::Edit;
                                        }
                                    });
                                }
                            });
                    });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(4.0);
            
            // Navigation with hover effects
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing.x = 6.0;
                
                let register_btn = ui.selectable_label(
                    self.current_section == AppSection::Register,
                    egui::RichText::new("📝 Skrá").size(16.0)
                );
                if register_btn.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if register_btn.clicked() {
                    self.current_section = AppSection::Register;
                }
                
                let edit_btn = ui.selectable_label(
                    self.current_section == AppSection::Edit,
                    egui::RichText::new("✏ Breyta").size(16.0)
                );
                if edit_btn.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if edit_btn.clicked() {
                    self.current_section = AppSection::Edit;
                }
                
                let search_btn = ui.selectable_label(
                    self.current_section == AppSection::Search,
                    egui::RichText::new("🔍 Leita").size(16.0)
                );
                if search_btn.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if search_btn.clicked() {
                    self.current_section = AppSection::Search;
                }
                
                let print_btn = ui.selectable_label(
                    self.current_section == AppSection::Print,
                    egui::RichText::new("📋 Prenta").size(16.0)
                );
                if print_btn.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if print_btn.clicked() {
                    self.current_section = AppSection::Print;
                }
            });
            
            ui.separator();
            
            // Messages with better visibility
            if !self.message.is_empty() {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("✅").color(egui::Color32::from_rgb(46, 125, 50)).size(20.0));
                    ui.label(egui::RichText::new(&self.message).color(egui::Color32::from_rgb(46, 125, 50)).strong());
                });
                ui.add_space(5.0);
            }
            if !self.error_message.is_empty() {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("❌").color(egui::Color32::from_rgb(211, 47, 47)).size(20.0));
                    ui.label(egui::RichText::new(&self.error_message).color(egui::Color32::from_rgb(211, 47, 47)).strong());
                });
                ui.add_space(5.0);
            }
            
            ui.add_space(10.0);
            
            // Content
            match self.current_section {
                AppSection::Register => self.register_section(ui),
                AppSection::Edit => self.edit_section(ui),
                AppSection::Search => self.search_section(ui),
                AppSection::Print => self.print_section(ui),
            }
        });

        // Persistent footer with author and year, improved contrast and margins
        egui::TopBottomPanel::bottom("app_footer")
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(60, 100, 140))
                    .inner_margin(egui::Margin::symmetric(16.0, 8.0))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(180)))
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Daníel Snær Rodríguez, 2025").color(egui::Color32::WHITE).strong());
                });
            });
    }
}
