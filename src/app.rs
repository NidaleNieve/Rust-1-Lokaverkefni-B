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
    // (removed old room/floor filters previously used for separate sections)

    // update location (use same widgets as new item)
    update_id: String,
    update_house: House,
    update_floor: u8,
    update_room: u16,

    // print by id
    print_id: String,

    // unified printing section
    print_scope: PrintScope,
    print_house: House,
    print_floor: u8,
    print_room: u16,
    print_kind: EquipmentKind,

    // pretty output area (bottom panel)
    print_output: Option<String>,
    print_details: Option<String>,
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
                print_scope: PrintScope::All,
                print_house: House::H,
                print_floor: 1,
                print_room: 1,
                print_kind: EquipmentKind::Table,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum PrintScope {
    All,
    ByHouse,
    ByKind,
    ByRoom,
    ByFloorInHouse,
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
            egui::ComboBox::from_label("Hús")
                .selected_text(self.state.update_house.to_string())
                .show_ui(ui, |ui| {
                    for h in [House::HA, House::H, House::S] {
                        ui.selectable_value(&mut self.state.update_house, h, h.to_string());
                    }
                });
            ui.add(egui::Slider::new(&mut self.state.update_floor, 0..=9).text("Hæð"));
            ui.add(egui::Slider::new(&mut self.state.update_room, 1..=999).text("Herbergi"));
            if ui.button("Uppfæra").clicked() {
                let id = self.state.update_id.parse::<i64>().unwrap_or(-1);
                let loc = Location { house: self.state.update_house, floor: self.state.update_floor, room: self.state.update_room };
                match self.inv.update_location(id, &loc) {
                    Ok(true) => { self.status = Some("Staðsetning uppfærð".into()); self.reload_items(); }
                    Ok(false) => self.status = Some("Ekkert fannst með þetta id".into()),
                    Err(e) => self.status = Some(format!("Villa: {}", e)),
                };
            }

            ui.separator();
            ui.heading("Prenta");
            // Togglable options
            egui::CollapsingHeader::new("Valkostur").default_open(true).show(ui, |ui| {
                ui.radio_value(&mut self.state.print_scope, PrintScope::All, "Allur búnaður");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.state.print_scope, PrintScope::ByHouse, "Eftir húsi");
                    if self.state.print_scope == PrintScope::ByHouse {
                        egui::ComboBox::from_label("Hús")
                            .selected_text(self.state.print_house.to_string())
                            .show_ui(ui, |ui| {
                                for h in [House::HA, House::H, House::S] {
                                    ui.selectable_value(&mut self.state.print_house, h, h.to_string());
                                }
                            });
                    }
                });
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.state.print_scope, PrintScope::ByKind, "Eftir tegund");
                    if self.state.print_scope == PrintScope::ByKind {
                        egui::ComboBox::from_label("Tegund")
                            .selected_text(self.state.print_kind.to_string())
                            .show_ui(ui, |ui| {
                                for k in [EquipmentKind::Table, EquipmentKind::Chair, EquipmentKind::Projector] {
                                    ui.selectable_value(&mut self.state.print_kind, k, k.to_string());
                                }
                            });
                    }
                });
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.state.print_scope, PrintScope::ByRoom, "Í ákveðinni stofu");
                    if self.state.print_scope == PrintScope::ByRoom {
                        egui::ComboBox::from_label("Hús")
                            .selected_text(self.state.print_house.to_string())
                            .show_ui(ui, |ui| {
                                for h in [House::HA, House::H, House::S] {
                                    ui.selectable_value(&mut self.state.print_house, h, h.to_string());
                                }
                            });
                        ui.add(egui::Slider::new(&mut self.state.print_floor, 0..=9).text("Hæð"));
                        ui.add(egui::Slider::new(&mut self.state.print_room, 1..=999).text("Herbergi"));
                    }
                });
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.state.print_scope, PrintScope::ByFloorInHouse, "Á ákveðinni hæð í húsi");
                    if self.state.print_scope == PrintScope::ByFloorInHouse {
                        egui::ComboBox::from_label("Hús")
                            .selected_text(self.state.print_house.to_string())
                            .show_ui(ui, |ui| {
                                for h in [House::HA, House::H, House::S] {
                                    ui.selectable_value(&mut self.state.print_house, h, h.to_string());
                                }
                            });
                        ui.add(egui::Slider::new(&mut self.state.print_floor, 0..=9).text("Hæð"));
                    }
                });
            });
            if ui.button("Prenta").clicked() {
                // Resolve selection and update list and a human-readable summary
                let res = match self.state.print_scope {
                    PrintScope::All => self.inv.all().map(|v| (v, "Allur búnaður".to_string())),
                    PrintScope::ByHouse => self.inv.by_house(self.state.print_house).map(|v| (v, format!("Búnaður í húsi {}", self.state.print_house))),
                    PrintScope::ByKind => self.inv.by_kind(self.state.print_kind).map(|v| (v, format!("Búnaður af tegund {}", self.state.print_kind))),
                    PrintScope::ByRoom => {
                        let loc = Location { house: self.state.print_house, floor: self.state.print_floor, room: self.state.print_room };
                        self.inv.by_room(&loc).map(|v| (v, format!("Búnaður í stofu {}", loc)))
                    }
                    PrintScope::ByFloorInHouse => {
                        let h = self.state.print_house; let f = self.state.print_floor;
                        self.inv.by_floor(h, f).map(|v| (v, format!("Búnaður á {}-hæð í {}", f, h)))
                    }
                };
                match res {
                    Ok((v, label)) => {
                        self.items = v;
                        self.state.print_output = Some(format!("{}: {} atriði.", label, self.items.len()));
                        // Pretty printed lines
                        let mut buf = String::new();
                        for rec in &self.items {
                            use std::fmt::Write as _;
                            let _ = writeln!(&mut buf, "{}", rec);
                        }
                        self.state.print_details = if buf.is_empty() { None } else { Some(buf) };
                    }
                    Err(e) => {
                        self.state.print_output = Some(format!("Villa við prentun: {}", e));
                        self.state.print_details = None;
                    }
                }
            }

            ui.separator();
            ui.heading("Prenta eftir ID");
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.state.print_id).hint_text("id"));
                if ui.button("Prenta").clicked() {
                    if let Ok(id) = self.state.print_id.parse::<i64>() {
                        match self.inv.by_id(id) {
                            Ok(Some(rec)) => {
                                self.state.print_output = Some("1 atriði".into());
                                self.state.print_details = Some(format!("{}", rec));
                            }
                            Ok(None) => { self.state.print_output = Some("Ekkert fannst".into()); self.state.print_details = None; }
                            Err(e) => { self.state.print_output = Some(format!("Villa: {}", e)); self.state.print_details = None; }
                        }
                    }
                }
            });

            ui.separator();
            ui.heading("Eyða eftir ID");
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.state.print_id).hint_text("id"));
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
                    .column(egui_extras::Column::initial(48.0))
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
                                row.col(|ui| {
                                    let txt = egui::RichText::new(item.id.unwrap_or_default().to_string()).monospace();
                                    ui.add(egui::Label::new(txt).wrap(false));
                                });
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

        // Pretty output area at the bottom (not grey/top)
        egui::TopBottomPanel::bottom("print_output_panel").show(ctx, |ui| {
            if let Some(txt) = &self.state.print_output {
                ui.group(|ui| {
                    ui.heading("Úttak");
                    ui.separator();
                    ui.label(egui::RichText::new(txt).strong());
                    if let Some(details) = &self.state.print_details {
                        ui.separator();
                        egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                            ui.monospace(details);
                        });
                    }
                });
            }
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
            update_id: String::new(),
            update_house: House::H,
            update_floor: 1,
            update_room: 1,
            print_id: String::new(),
            print_scope: PrintScope::All,
            print_house: House::H,
            print_floor: 1,
            print_room: 1,
            print_kind: EquipmentKind::Table,
            print_output: None,
            print_details: None,
        }
    }
}