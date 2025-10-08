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

#[derive(PartialEq, Clone, Copy)]
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
    // Command palette
    show_palette: bool,
    palette_query: String,
    palette_focus_on_open: bool,
    // Command palette selection state
    palette_selected_index: Option<usize>,
    // For animated stats
    _stats_anim_seen: bool,
}

impl EquipmentApp {
    // Very small Levenshtein implementation for fuzzy matching (O(n*m), fine for tiny inputs)
    fn levenshtein(a: &str, b: &str) -> usize {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let n = a_bytes.len();
        let m = b_bytes.len();
        if n == 0 { return m; }
        if m == 0 { return n; }
        let mut prev: Vec<usize> = (0..=m).collect();
        let mut curr: Vec<usize> = vec![0; m + 1];
        for i in 1..=n {
            curr[0] = i;
            for j in 1..=m {
                let cost = if a_bytes[i - 1].to_ascii_lowercase() == b_bytes[j - 1].to_ascii_lowercase() { 0 } else { 1 };
                curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
            }
            std::mem::swap(&mut prev, &mut curr);
        }
        prev[m]
    }
    // Draw a radio button with black fill when selected, preserving label color
    fn radio_black_value<T: PartialEq + Copy>(ui: &mut egui::Ui, value: &mut T, selected: T, label: &str) {
        ui.horizontal(|ui| {
            let size = ui.spacing().interact_size.y;
            let (rect, resp) = ui.allocate_at_least(egui::vec2(size, size), egui::Sense::click());
            let is_selected = *value == selected;
            if resp.clicked() { *value = selected; }
            // Paint black-dot radio
            let center = rect.center();
            let outer_r = rect.height() * 0.35;
            let inner_r = outer_r * 0.45; // slightly smaller for clearer ring
            // White-filled ring with black outline for strong contrast on gray
            ui.painter().circle_filled(center, outer_r, egui::Color32::WHITE);
            ui.painter().circle_stroke(center, outer_r, egui::Stroke { width: 1.5, color: egui::Color32::BLACK });
            if is_selected {
                ui.painter().circle_filled(center, inner_r, egui::Color32::BLACK);
            }
            ui.add_space(6.0);
            let label_resp = ui.add(egui::Label::new(label).sense(egui::Sense::click()));
            if label_resp.clicked() { *value = selected; }
        });
    }

    // Parse and execute palette command; return true if the palette should close
    fn run_palette_command(&mut self, cmd: &str) -> bool {
        // ...existing methods...
        let c = cmd.trim();
        if c.is_empty() { return false; }
        let lower = c.to_lowercase();

        // Quick help
        if lower == "hjálp" || lower == "help" {
            self.message = "Dæmi: 'sækja 12', 'eyða 12', 'skrá HA-101', 'uppfæra 12 í HA-101', 'leita stóll'".into();
            return true;
        }

        // Navigation
        if lower.starts_with("fara í ") {
            let rest = lower[7..].trim();
            match rest {
                "skrá" => { self.current_section = AppSection::Register; return true; },
                "breyta" => { self.current_section = AppSection::Edit; return true; },
                "leita" => { self.current_section = AppSection::Search; return true; },
                "prenta" => { self.current_section = AppSection::Print; return true; },
                _ => { self.error_message = format!("Óþekkt leið: {}", rest); return false; }
            }
        }

        // Toggle sidebar
        if lower == "opna lista" { self.show_sidebar = true; return true; }
        if lower == "fela lista" { self.show_sidebar = false; return true; }

        // Print / PDF
        if lower == "prenta" { self.print_current_list(); return true; }
        if lower == "pdf" || lower == "flytja út pdf" { self.export_current_list_pdf(); return true; }

        // JSON
        if lower == "vista json" { self.save_to_json(); return true; }
        if lower == "hlaða json" { self.load_from_json(); return true; }

        // Tutorial commands removed

        // sækja <id>
        if lower.starts_with("sækja ") {
            if let Some(id_str) = lower.split_whitespace().nth(1) {
                if id_str.chars().all(|c| c.is_ascii_digit()) {
                    self.edit_id = id_str.to_string();
                    self.fetch_equipment_for_edit();
                    self.current_section = AppSection::Edit;
                    return true;
                }
            }
            self.error_message = "Notkun: sækja <id>".into();
            return false;
        }

        // eyða <id>
        if lower.starts_with("eyða ") || lower.starts_with("eyda ") {
            let id_str = lower.split_whitespace().nth(1).unwrap_or("");
            if id_str.chars().all(|c| c.is_ascii_digit()) {
                self.edit_id = id_str.to_string();
                self.fetch_equipment_for_edit();
                self.delete_equipment();
                return true;
            }
            self.error_message = "Notkun: eyða <id>".into();
            return false;
        }

        // leita <text>
        if lower.starts_with("leita ") {
            let q = c[6..].trim();
            self.search_query = q.to_string();
            self.perform_search();
            self.current_section = AppSection::Search;
            return true;
        }

        // skrá <code>  where code is like HA-101 or S-569 -> S 5th floor room 69
        if lower.starts_with("skrá ") || lower.starts_with("skra ") {
            let code = c.split_whitespace().nth(1).unwrap_or("");
            if let Ok(loc) = Location::try_from(code) {
                self.current_section = AppSection::Register;
                self.reg_building = loc.building;
                self.reg_floor = loc.floor;
                self.reg_room = loc.room;
                return true;
            } else {
                self.error_message = "Notkun: skrá <HA|H|S>-<hæð><stofa> t.d. S-569".into();
                return false;
            }
        }

        // uppfæra <id> í <code>
        if lower.starts_with("uppfæra ") || lower.starts_with("uppfaera ") {
            let parts: Vec<&str> = lower.split_whitespace().collect();
            if parts.len() >= 3 {
                let id_str = parts[1];
                // Find the location code after 'í' or 'i'
                let mut code_opt: Option<&str> = None;
                for w in parts.iter().skip(2) {
                    if *w == "í" || *w == "i" { continue; }
                    code_opt = Some(*w);
                    break;
                }
                if let (true, Some(code)) = (id_str.chars().all(|c| c.is_ascii_digit()), code_opt) {
                    if let Ok(loc) = Location::try_from(code) {
                        self.current_section = AppSection::Edit;
                        self.edit_id = id_str.to_string();
                        self.fetch_equipment_for_edit();
                        // Apply update
                        self.edit_building = loc.building;
                        self.edit_floor = loc.floor;
                        self.edit_room = loc.room;
                        self.update_location();
                        return true;
                    }
                }
            }
            self.error_message = "Notkun: uppfæra <id> í <HA|H|S>-<hæð><stofa>".into();
            return false;
        }

        // direct section keywords
        match lower.as_str() {
            "skrá" | "skra" => { self.current_section = AppSection::Register; true },
            "breyta" => { self.current_section = AppSection::Edit; true },
            "leita" => { self.current_section = AppSection::Search; true },
            "prenta" => { self.current_section = AppSection::Print; true },
            _ => { self.error_message = format!("Óþekkt skipun: {}", c); false }
        }
    }
    // Subtle animated button wrapper: adds hover cursor and a gentle overlay
    fn button_animated(
        ui: &mut egui::Ui,
        text: impl Into<egui::WidgetText>,
    ) -> egui::Response {
        let rounding = ui.style().visuals.widgets.inactive.rounding;
        let tint = egui::Color32::from_white_alpha(12);
        let resp = ui.add(egui::Button::new(text));
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            let rect = resp.rect;
            ui.painter().rect_filled(rect, rounding, tint);
        }
        resp
    }

    // Clickable cell with hover highlight; sized to fit table cells
    fn clickable_cell(
        ui: &mut egui::Ui,
        text: impl Into<egui::WidgetText>,
        size: [f32; 2],
    ) -> egui::Response {
        let rounding = ui.style().visuals.widgets.inactive.rounding;
        let tint = egui::Color32::from_white_alpha(18);
        let resp = ui.add_sized(size, egui::Label::new(text).sense(egui::Sense::click()));
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            let rect = resp.rect;
            ui.painter().rect_filled(rect, rounding, tint);
        }
        resp
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
            show_palette: false,
            palette_query: String::new(),
            palette_focus_on_open: false,
            palette_selected_index: None,
            _stats_anim_seen: false,
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
            if Self::button_animated(ui, "🔍 Sækja").clicked() {
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
            if Self::button_animated(ui, "✅ Uppfæra staðsetningu").clicked() {
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
            
            if Self::button_animated(ui, "❌ Eyða búnaði").clicked() {
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
                                row.col(|ui| { if Self::clickable_cell(ui, id.to_string(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                row.col(|ui| { if Self::clickable_cell(ui, equipment.get_type_name().to_string(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                row.col(|ui| { if Self::clickable_cell(ui, location_str.clone(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                row.col(|ui| { if Self::clickable_cell(ui, format!("{} kr.", value), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                row.col(|ui| { if Self::clickable_cell(ui, format!("{}", equipment), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });

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
            if Self::button_animated(ui, "💾 Vista í JSON").clicked() {
                self.save_to_json();
            }
            if Self::button_animated(ui, "📂 Hlaða úr JSON").clicked() {
                self.load_from_json();
            }
            ui.add_space(12.0);
            if Self::button_animated(ui, "📄 Prenta lista").clicked() {
                self.print_current_list();
            }
            if Self::button_animated(ui, "💾 Flytja út í PDF").clicked() {
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
                                row.col(|ui| { if Self::clickable_cell(ui, id.to_string(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                row.col(|ui| { if Self::clickable_cell(ui, equipment.get_type_name(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                row.col(|ui| { if Self::clickable_cell(ui, location_str.clone(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                row.col(|ui| { if Self::clickable_cell(ui, format!("{} kr.", value), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                row.col(|ui| { if Self::clickable_cell(ui, format!("{}", equipment), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });

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
                        if Self::button_animated(ui, egui::RichText::new(if self.show_sidebar { format!("{arrow} Fela lista") } else { "📋 Sjá lista".into() }).color(egui::Color32::WHITE)).clicked() {
                            self.show_sidebar = !self.show_sidebar; 
                            // Refresh sidebar data when opening
                            if self.show_sidebar {
                                self.display_filter = DisplayFilter::All;
                                self.load_equipment();
                            }
                        }
                        ui.add_space(8.0);
                        if Self::button_animated(ui, egui::RichText::new("⌘K Skipunir").color(egui::Color32::WHITE)).clicked() {
                            self.show_palette = true;
                            self.palette_query.clear();
                            self.palette_focus_on_open = true;
                        }
                        // Tutorial button removed
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

        // Keyboard: Command Palette toggle (Cmd+K)
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::K)) {
            self.show_palette = !self.show_palette;
            self.palette_query.clear();
            if self.show_palette { self.palette_focus_on_open = true; }
        }

        // Keyboard: Esc closes palette when open
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            if self.show_palette { self.show_palette = false; }
        }

        if self.show_palette {
            egui::Window::new("Valmynd")
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .show(ctx, |ui| {
                    ui.set_min_width(420.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Valmynd").strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if Self::button_animated(ui, "❌").clicked() { self.show_palette = false; }
                        });
                    });
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.palette_query)
                            .hint_text("t.d. \"eyða 23\", \"skrá HA-123\", \"leita stóll\"")
                    );
                    if self.palette_focus_on_open { resp.request_focus(); self.palette_focus_on_open = false; }
                    ui.add_space(6.0);
                    // Suggestions with fuzzy ranking
                    let q = self.palette_query.trim().to_lowercase();
                    let base: [(&str, &str); 15] = [
                        ("Fara í Skrá", "fara í skrá"),
                        ("Fara í Breyta", "fara í breyta"),
                        ("Fara í Leita", "fara í leita"),
                        ("Fara í Prenta", "fara í prenta"),
                        ("Sækja ID", "sækja 123"),
                        ("Eyða ID", "eyða 123"),
                        ("Skrá staðsetningu", "skrá HA-101"),
                        ("Uppfæra staðsetningu ID", "uppfæra 123 í HA-101"),
                        ("Leita", "leita stóll"),
                        ("Prenta", "prenta"),
                        ("Flytja út PDF", "pdf"),
                        ("Vista JSON", "vista json"),
                        ("Hlaða JSON", "hlaða json"),
                        ("Opna lista (hægri)", "opna lista"),
                        ("Fela lista (hægri)", "fela lista"),
                    ];
                    let mut items: Vec<(String, String, usize)> = base.iter().map(|(l,c)| {
                        let label = l.to_string();
                        let cmd = c.to_string();
                        let key = format!("{} {}", label.to_lowercase(), cmd.to_lowercase());
                        let score = if q.is_empty() { 0 } else { Self::levenshtein(&q, &key) };
                        (label, cmd, score)
                    }).collect();
                    items.sort_by_key(|(_,_,s)| *s);
                    // Keep only decent matches if user typed something (distance threshold)
                    if !q.is_empty() {
                        items.retain(|(l,c,s)| {
                            let min_len = q.len().min(l.len().min(c.len()));
                            *s <= (min_len / 2).max(1)
                                || l.to_lowercase().contains(&q)
                                || c.to_lowercase().contains(&q)
                        });
                    }
                    // Selection with keys: Tab/Shift+Tab or arrows
                    let total = items.len();
                    if total > 0 {
                        // Initialize selection
                        if self.palette_selected_index.is_none() { self.palette_selected_index = Some(0); }
                        let mut idx = self.palette_selected_index.unwrap_or(0).min(total - 1);
                        let next = |i: usize, t: usize| if i + 1 >= t { 0 } else { i + 1 };
                        let prev = |i: usize, t: usize| if i == 0 { t - 1 } else { i - 1 };
                        let (tab, shift) = ui.input(|i| (i.key_pressed(egui::Key::Tab), i.modifiers.shift));
                        if tab {
                            idx = if shift { prev(idx, total) } else { next(idx, total) };
                        }
                        let (down, up) = ui.input(|i| (i.key_pressed(egui::Key::ArrowDown), i.key_pressed(egui::Key::ArrowUp)));
                        if down { idx = next(idx, total); }
                        if up { idx = prev(idx, total); }
                        self.palette_selected_index = Some(idx);

                        // Enter: run highlighted suggestion if present; otherwise run raw query
                        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            if let Some((_, cmd, _)) = items.get(idx).cloned() {
                                if self.run_palette_command(cmd.as_str()) { self.show_palette = false; }
                            } else {
                                let cmd = self.palette_query.clone();
                                if self.run_palette_command(cmd.trim()) { self.show_palette = false; }
                            }
                        }

                        // Render suggestions with highlight
                        for (i, (label, cmd, _score)) in items.iter().enumerate() {
                            let is_sel = Some(i) == self.palette_selected_index;
                            let text = if is_sel { format!("▶ {}", label) } else { label.clone() };
                            let clicked = Self::button_animated(ui, text).clicked();
                            if clicked {
                                if self.run_palette_command(cmd) { self.show_palette = false; }
                            }
                        }
                    } else {
                        // No items: allow Enter to run raw command for power users
                        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let cmd = self.palette_query.clone();
                            if self.run_palette_command(cmd.trim()) { self.show_palette = false; }
                        }
                        ui.label(egui::RichText::new("Engar tillögur").italics());
                    }
                });
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
                                        row.col(|ui| { if Self::clickable_cell(ui, id.to_string(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                        row.col(|ui| { if Self::clickable_cell(ui, equipment.get_type_name(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                        row.col(|ui| { if Self::clickable_cell(ui, location_str.clone(), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                        row.col(|ui| { if Self::clickable_cell(ui, format!("{} kr.", value), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
                                        if show_description {
                                            row.col(|ui| { if Self::clickable_cell(ui, format!("{}", equipment), [ui.available_width(), row_h]).clicked() { clicked_any = true; } });
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

    // Tutorial overlay removed

}

}
