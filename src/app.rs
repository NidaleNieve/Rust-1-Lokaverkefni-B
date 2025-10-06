use crate::inventory::Inventory;
use crate::models::*;
use thousands::Separable;

pub struct BunadarApp {
    inv: Inventory,
    state: UiState,
    items: Vec<EquipmentRecord>,
    filter: Filter,
    status: Option<String>,
}

struct UiState {
    // Form inputs for adding item
    new_kind: EquipmentKind,
    new_value_isk: i64,
    new_house: House,
    new_floor: u8,
    new_room: u16,
    // Specifics
    new_seats: u8,
    new_chair_kind: ChairKind,
    new_lumens: u32,

    // update location
    // filters by specific room/floor
    filter_house: House,
    filter_floor: u8,
    filter_room: u16,

    // update location
    update_id: String,
    update_location: String,

    // print by id
    print_id: String,
}

#[derive(Default, Clone, Copy)]
struct Filter {
    by_house: Option<House>,
    by_kind: Option<EquipmentKind>,
}

impl BunadarApp {
    pub fn new(inv: Inventory) -> Self {
        let items = inv.all().unwrap_or_default();
        Self {
            inv,
            state: UiState {
                new_value_isk: 0,
                new_chair_kind: ChairKind::Annad,
                filter_house: House::H,
                filter_floor: 1,
                filter_room: 1,
                ..Default::default()
            },
            items,
            filter: Filter::default(),
            status: None,
        }
    }

    fn reload_items(&mut self) {
        let res = match (self.filter.by_house, self.filter.by_kind) {
            (Some(h), None) => self.inv.by_house(h),
            (None, Some(k)) => self.inv.by_kind(k),
            (Some(h), Some(_k)) => self.inv.by_house(h), // minimal: house dominates
            (None, None) => self.inv.all(),
        };
        self.items = res.unwrap_or_default();
    }
}

impl eframe::App for BunadarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.heading("Búnaðarlisti Tækniskólans");
            ui.horizontal(|ui| {
                ui.label("Sía:");
                egui::ComboBox::from_label("Hús")
                    .selected_text(self.filter.by_house.map(|h| h.to_string()).unwrap_or_else(|| "Allt".into()))
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(self.filter.by_house.is_none(), "Allt").clicked() {
                            self.filter.by_house = None;
                            self.reload_items();
                        }
                        for h in [House::HA, House::H, House::S] {
                            if ui.selectable_label(self.filter.by_house == Some(h), h.to_string()).clicked() {
                                self.filter.by_house = Some(h);
                                self.reload_items();
                            }
                        }
                    });
                egui::ComboBox::from_label("Tegund")
                    .selected_text(self.filter.by_kind.map(|k| k.to_string()).unwrap_or_else(|| "Allt".into()))
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(self.filter.by_kind.is_none(), "Allt").clicked() {
                            self.filter.by_kind = None;
                            self.reload_items();
                        }
                        for k in [EquipmentKind::Table, EquipmentKind::Chair, EquipmentKind::Projector] {
                            if ui.selectable_label(self.filter.by_kind == Some(k), k.to_string()).clicked() {
                                self.filter.by_kind = Some(k);
                                self.reload_items();
                            }
                        }
                    });
                if ui.button("Endurlesa").clicked() { self.reload_items(); }
                if ui.button("Flytja út JSON").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).save_file() {
                        let _ = self.inv.export_json(path.to_str().unwrap()).map(|n| {
                            self.status = Some(format!("Vistaði {} hluti í JSON.", n));
                        }).map_err(|e| {
                            self.status = Some(format!("Villa við útflutning: {}", e));
                        });
                    }
                }
                if ui.button("Flytja inn JSON").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
                        match self.inv.import_json(path.to_str().unwrap()) {
                            Ok(n) => {
                                self.status = Some(format!("Flutti inn {} hluti.", n));
                                self.reload_items();
                            }
                            Err(e) => self.status = Some(format!("Villa við innflutning: {}", e)),
                        }
                    }
                }
            });
            if let Some(s) = &self.status {
                ui.label(egui::RichText::new(s).italics());
            }
        });

        egui::SidePanel::left("left").resizable(true).show(ctx, |ui| {
            ui.heading("Nýr hlutur");
            ui.separator();

            egui::ComboBox::from_label("Tegund")
                .selected_text(self.state.new_kind.to_string())
                .show_ui(ui, |ui| {
                    for k in [EquipmentKind::Table, EquipmentKind::Chair, EquipmentKind::Projector] {
                        ui.selectable_value(&mut self.state.new_kind, k, k.to_string());
                    }
                });

            ui.add(egui::DragValue::new(&mut self.state.new_value_isk).speed(1000).prefix("Verðmæti (kr): "));

            egui::ComboBox::from_label("Hús")
                .selected_text(self.state.new_house.to_string())
                .show_ui(ui, |ui| {
                    for h in [House::HA, House::H, House::S] {
                        ui.selectable_value(&mut self.state.new_house, h, h.to_string());
                    }
                });
            ui.add(egui::Slider::new(&mut self.state.new_floor, 0..=9).text("Hæð"));
            ui.add(egui::Slider::new(&mut self.state.new_room, 1..=999).text("Herbergi"));

            match self.state.new_kind {
                EquipmentKind::Table => {
                    ui.add(egui::Slider::new(&mut self.state.new_seats, 1..=12).text("Sæti"));
                }
                EquipmentKind::Chair => {
                    egui::ComboBox::from_label("Gerð stóls")
                        .selected_text(self.state.new_chair_kind.to_string())
                        .show_ui(ui, |ui| {
                            for ck in [ChairKind::Haegindastoll, ChairKind::Skolastoll, ChairKind::Skrifstofustoll, ChairKind::Annad] {
                                ui.selectable_value(&mut self.state.new_chair_kind, ck, ck.to_string());
                            }
                        });
                }
                EquipmentKind::Projector => {
                    ui.add(egui::DragValue::new(&mut self.state.new_lumens).speed(100).prefix("Lumens: "));
                }
            }

            if ui.button("Skrá í lista").clicked() {
                let loc = Location { house: self.state.new_house, floor: self.state.new_floor, room: self.state.new_room };
                let rec = match self.state.new_kind {
                    EquipmentKind::Table => EquipmentRecord {
                        id: None,
                        kind: EquipmentKind::Table,
                        value_isk: self.state.new_value_isk.max(0),
                        location: loc,
                        seats: Some(self.state.new_seats.max(1)),
                        chair_kind: None,
                        lumens: None,
                    },
                    EquipmentKind::Chair => EquipmentRecord {
                        id: None,
                        kind: EquipmentKind::Chair,
                        value_isk: self.state.new_value_isk.max(0),
                        location: loc,
                        seats: None,
                        chair_kind: Some(self.state.new_chair_kind),
                        lumens: None,
                    },
                    EquipmentKind::Projector => EquipmentRecord {
                        id: None,
                        kind: EquipmentKind::Projector,
                        value_isk: self.state.new_value_isk.max(0),
                        location: loc,
                        seats: None,
                        chair_kind: None,
                        lumens: Some(self.state.new_lumens.max(100)),
                    },
                };
                match self.inv.add(rec) {
                    Ok(id) => {
                        self.status = Some(format!("Skráð! Nýtt id = {}", id));
                        self.reload_items();
                    }
                    Err(e) => self.status = Some(format!("Villa: {}", e)),
                }
            }

            ui.separator();
            ui.heading("Uppfæra staðsetningu");
            ui.add(egui::TextEdit::singleline(&mut self.state.update_id).hint_text("id"));
            ui.add(egui::TextEdit::singleline(&mut self.state.update_location).hint_text("t.d. H-202"));
            if ui.button("Uppfæra").clicked() {
                let id = self.state.update_id.parse::<i64>().unwrap_or(-1);
                match self.state.update_location.parse::<Location>() {
                    Ok(loc) => match self.inv.update_location(id, &loc) {
                        Ok(true) => { self.status = Some("Staðsetning uppfærð".into()); self.reload_items(); }
                        Ok(false) => self.status = Some("Ekkert fannst með þetta id".into()),
                        Err(e) => self.status = Some(format!("Villa: {}", e)),
                    },
                    Err(e) => self.status = Some(format!("Ógild staðsetning: {}", e)),
                }
            }

            
            ui.separator();
            ui.heading("Birta búnað í ákveðinni stofu");
            egui::ComboBox::from_label("Hús (stofa)")
                .selected_text(self.state.filter_house.to_string())
                .show_ui(ui, |ui| {
                    for h in [House::HA, House::H, House::S] {
                        ui.selectable_value(&mut self.state.filter_house, h, h.to_string());
                    }
                });
            ui.add(egui::Slider::new(&mut self.state.filter_floor, 0..=9).text("Hæð (stofa)"));
            ui.add(egui::Slider::new(&mut self.state.filter_room, 1..=999).text("Herbergi (stofa)"));
            if ui.button("Birta stofu").clicked() {
                let loc = Location { house: self.state.filter_house, floor: self.state.filter_floor, room: self.state.filter_room };
                match self.inv.by_room(&loc) {
                    Ok(v) => { self.items = v; self.status = Some(format!("{} hlutir í {}", self.items.len(), loc)); }
                    Err(e) => self.status = Some(format!("Villa: {}", e)),
                }
            }

            ui.separator();
            ui.heading("Birta búnað á ákveðinni hæð í húsi");
            let mut house2 = self.state.filter_house;
            egui::ComboBox::from_label("Hús (hæð)")
                .selected_text(house2.to_string())
                .show_ui(ui, |ui| {
                    for h in [House::HA, House::H, House::S] {
                        ui.selectable_value(&mut house2, h, h.to_string());
                    }
                });
            let mut floor2 = self.state.filter_floor;
            ui.add(egui::Slider::new(&mut floor2, 0..=9).text("Hæð (hús)"));
            if ui.button("Birta hæð").clicked() {
                match self.inv.by_floor(house2, floor2) {
                    Ok(v) => { self.items = v; self.status = Some(format!("{} hlutir á {}-hæð í {}", self.items.len(), floor2, house2)); }
                    Err(e) => self.status = Some(format!("Villa: {}", e)),
                }
            }

            ui.separator();
            ui.heading("Prenta/eyða");
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.state.print_id).hint_text("id"));
                if ui.button("Prenta").clicked() {
                    if let Ok(id) = self.state.print_id.parse::<i64>() {
                        match self.inv.by_id(id) {
                            Ok(Some(rec)) => self.status = Some(format!("{}", rec)),
                            Ok(None) => self.status = Some("Ekkert fannst".into()),
                            Err(e) => self.status = Some(format!("Villa: {}", e)),
                        }
                    }
                }
                if ui.button("Eyða").clicked() {
                    if let Ok(id) = self.state.print_id.parse::<i64>() {
                        match self.inv.remove(id) {
                            Ok(true) => { self.status = Some("Eytt".into()); self.reload_items(); }
                            Ok(false) => self.status = Some("Ekkert fannst".into()),
                            Err(e) => self.status = Some(format!("Villa: {}", e)),
                        }
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Listi");
            ui.separator();

            let text_height = egui::TextStyle::Body.resolve(ui.style()).size * 1.2;
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::remainder())
                    .header(text_height, |mut row| {
                        row.col(|ui| { ui.strong("Id"); });
                        row.col(|ui| { ui.strong("Tegund"); });
                        row.col(|ui| { ui.strong("Staðsetning"); });
                        row.col(|ui| { ui.strong("Verðmæti"); });
                        row.col(|ui| { ui.strong("Auka-upplýsingar"); });
                    })
                    .body(|mut body| {
                        for item in &self.items {
                            body.row(text_height, |mut row| {
                                row.col(|ui| { ui.label(item.id.unwrap_or_default().to_string()); });
                                row.col(|ui| { ui.label(item.kind.to_string()); });
                                row.col(|ui| { ui.label(item.location.to_string()); });
                                row.col(|ui| { ui.label(format!("{} kr.", item.value_isk.separate_with_spaces())); });
                                row.col(|ui| {
                                    match item.kind {
                                        EquipmentKind::Table => ui.label(format!("Sæti: {}", item.seats.unwrap_or_default())),
                                        EquipmentKind::Chair => ui.label(format!("Gerð: {}", item.chair_kind.map(|c| c.to_string()).unwrap_or_else(|| "-".into()))),
                                        EquipmentKind::Projector => ui.label(format!("Lumens: {}", item.lumens.unwrap_or_default())),
                                    };
                                });
                            });
                        }
                    });
            });
        });
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            new_kind: EquipmentKind::Table,
            new_value_isk: 0,
            new_house: House::H,
            new_floor: 1,
            new_room: 1,
            new_seats: 4,
            new_chair_kind: ChairKind::Annad,
            new_lumens: 1000,
            filter_house: House::H,
            filter_floor: 1,
            filter_room: 1,
            update_id: String::new(),
            update_location: String::new(),
            print_id: String::new(),
        }
    }
}