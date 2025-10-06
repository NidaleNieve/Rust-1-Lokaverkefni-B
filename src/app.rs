use crate::inventory::Inventory;
use crate::models::*;
use thousands::Separable;

pub struct BunadarApp {
    inv: Inventory,
    state: UiState,
    items: Vec<EquipmentRecord>,
    filter: Filter,
    status: Option<String>,
    print_results: Vec<String>,
    print_title: Option<String>,
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
    update_house: House,
    update_floor: u8,
    update_room: u16,

    // update location
    update_id: String,

    // print by id
    print_id: String,

    // printing options
    filter_house: House,
    filter_floor: u8,
    filter_room: u16,
    filter_kind: EquipmentKind,
    print_mode: PrintMode,
}

#[derive(Default, Clone, Copy)]
struct Filter {
    by_house: Option<House>,
    by_kind: Option<EquipmentKind>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PrintMode {
    All,
    ByHouse,
    ByKind,
    ByRoom,
    ByFloor,
}

impl Default for PrintMode {
    fn default() -> Self {
        PrintMode::All
    }
}

impl PrintMode {
    fn label(&self) -> &'static str {
        match self {
            PrintMode::All => "Allur búnaður",
            PrintMode::ByHouse => "Búnaður eftir húsi",
            PrintMode::ByKind => "Búnaður eftir tegund",
            PrintMode::ByRoom => "Búnaður í stofu",
            PrintMode::ByFloor => "Búnaður á hæð",
        }
    }
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
                filter_kind: EquipmentKind::Table,
                update_house: House::H,
                update_floor: 1,
                update_room: 1,
                ..Default::default()
            },
            items,
            filter: Filter::default(),
            status: None,
            print_results: Vec::new(),
            print_title: None,
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

    fn set_print_data(&mut self, title: impl Into<String>, records: Vec<EquipmentRecord>) {
        let title = title.into();
        if records.is_empty() {
            self.print_results = vec!["Enginn búnaður fannst.".into()];
        } else {
            self.print_results = records.iter().map(Self::describe_record).collect();
        }
        self.print_title = Some(title);
    }

    fn describe_record(rec: &EquipmentRecord) -> String {
        rec.pretty_description()
    }
}

impl eframe::App for BunadarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.heading("Búnaðarlisti Tækniskólans");
            ui.horizontal(|ui| {
                ui.label("Sía:");
                egui::ComboBox::from_label("Hús")
                    .selected_text(
                        self.filter
                            .by_house
                            .map(|h| h.to_string())
                            .unwrap_or_else(|| "Allt".into()),
                    )
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(self.filter.by_house.is_none(), "Allt")
                            .clicked()
                        {
                            self.filter.by_house = None;
                            self.reload_items();
                        }
                        for h in [House::HA, House::H, House::S] {
                            if ui
                                .selectable_label(self.filter.by_house == Some(h), h.to_string())
                                .clicked()
                            {
                                self.filter.by_house = Some(h);
                                self.reload_items();
                            }
                        }
                    });
                egui::ComboBox::from_label("Tegund")
                    .selected_text(
                        self.filter
                            .by_kind
                            .map(|k| k.friendly_name().to_string())
                            .unwrap_or_else(|| "Allt".into()),
                    )
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(self.filter.by_kind.is_none(), "Allt")
                            .clicked()
                        {
                            self.filter.by_kind = None;
                            self.reload_items();
                        }
                        for k in [
                            EquipmentKind::Table,
                            EquipmentKind::Chair,
                            EquipmentKind::Projector,
                        ] {
                            if ui
                                .selectable_label(self.filter.by_kind == Some(k), k.friendly_name())
                                .clicked()
                            {
                                self.filter.by_kind = Some(k);
                                self.reload_items();
                            }
                        }
                    });
                if ui.button("Endurlesa").clicked() {
                    self.reload_items();
                }
                if ui.button("Flytja út JSON").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .save_file()
                    {
                        let _ = self
                            .inv
                            .export_json(path.to_str().unwrap())
                            .map(|n| {
                                self.status = Some(format!("Vistaði {} hluti í JSON.", n));
                            })
                            .map_err(|e| {
                                self.status = Some(format!("Villa við útflutning: {}", e));
                            });
                    }
                }
                if ui.button("Flytja inn JSON").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .pick_file()
                    {
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
                ui.label(
                    egui::RichText::new(s)
                        .strong()
                        .color(egui::Color32::from_rgb(200, 180, 40)),
                );
            }
        });

        egui::SidePanel::left("left")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Nýr hlutur");
                ui.separator();

                egui::ComboBox::from_label("Tegund")
                    .selected_text(self.state.new_kind.friendly_name())
                    .show_ui(ui, |ui| {
                        for k in [
                            EquipmentKind::Table,
                            EquipmentKind::Chair,
                            EquipmentKind::Projector,
                        ] {
                            ui.selectable_value(&mut self.state.new_kind, k, k.friendly_name());
                        }
                    });

                ui.add(
                    egui::DragValue::new(&mut self.state.new_value_isk)
                        .speed(1000)
                        .prefix("Verðmæti (kr): "),
                );

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
                                for ck in [
                                    ChairKind::Haegindastoll,
                                    ChairKind::Skolastoll,
                                    ChairKind::Skrifstofustoll,
                                    ChairKind::Annad,
                                ] {
                                    ui.selectable_value(
                                        &mut self.state.new_chair_kind,
                                        ck,
                                        ck.to_string(),
                                    );
                                }
                            });
                    }
                    EquipmentKind::Projector => {
                        ui.add(
                            egui::DragValue::new(&mut self.state.new_lumens)
                                .speed(100)
                                .prefix("Lumens: "),
                        );
                    }
                }

                if ui.button("Skrá í lista").clicked() {
                    let loc = Location {
                        house: self.state.new_house,
                        floor: self.state.new_floor,
                        room: self.state.new_room,
                    };
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
                ui.add(egui::TextEdit::singleline(&mut self.state.update_id).hint_text("ID"));
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
                    match self.state.update_id.trim().parse::<i64>() {
                        Ok(id) if id > 0 => {
                            let loc = Location {
                                house: self.state.update_house,
                                floor: self.state.update_floor,
                                room: self.state.update_room,
                            };
                            match self.inv.update_location(id, &loc) {
                                Ok(true) => {
                                    self.status = Some("Staðsetning uppfærð.".into());
                                    self.reload_items();
                                }
                                Ok(false) => {
                                    self.status = Some("Ekkert fannst með þetta ID.".into())
                                }
                                Err(e) => self.status = Some(format!("Villa: {}", e)),
                            }
                        }
                        _ => self.status = Some("ID þarf að vera jákvæð tala.".into()),
                    }
                }

                ui.separator();
                ui.heading("Prenta búnað");
                egui::ComboBox::from_label("Valmöguleiki")
                    .selected_text(self.state.print_mode.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.state.print_mode,
                            PrintMode::All,
                            PrintMode::All.label(),
                        );
                        ui.selectable_value(
                            &mut self.state.print_mode,
                            PrintMode::ByHouse,
                            PrintMode::ByHouse.label(),
                        );
                        ui.selectable_value(
                            &mut self.state.print_mode,
                            PrintMode::ByKind,
                            PrintMode::ByKind.label(),
                        );
                        ui.selectable_value(
                            &mut self.state.print_mode,
                            PrintMode::ByRoom,
                            PrintMode::ByRoom.label(),
                        );
                        ui.selectable_value(
                            &mut self.state.print_mode,
                            PrintMode::ByFloor,
                            PrintMode::ByFloor.label(),
                        );
                    });

                match self.state.print_mode {
                    PrintMode::All => {
                        ui.label("Prentar alla hluti í gagnagrunninum.");
                    }
                    PrintMode::ByHouse => {
                        egui::ComboBox::from_label("Hús")
                            .selected_text(self.state.filter_house.to_string())
                            .show_ui(ui, |ui| {
                                for h in [House::HA, House::H, House::S] {
                                    ui.selectable_value(
                                        &mut self.state.filter_house,
                                        h,
                                        h.to_string(),
                                    );
                                }
                            });
                    }
                    PrintMode::ByKind => {
                        egui::ComboBox::from_label("Tegund")
                            .selected_text(self.state.filter_kind.friendly_name())
                            .show_ui(ui, |ui| {
                                for k in [
                                    EquipmentKind::Table,
                                    EquipmentKind::Chair,
                                    EquipmentKind::Projector,
                                ] {
                                    ui.selectable_value(
                                        &mut self.state.filter_kind,
                                        k,
                                        k.friendly_name(),
                                    );
                                }
                            });
                    }
                    PrintMode::ByRoom => {
                        egui::ComboBox::from_label("Hús")
                            .selected_text(self.state.filter_house.to_string())
                            .show_ui(ui, |ui| {
                                for h in [House::HA, House::H, House::S] {
                                    ui.selectable_value(
                                        &mut self.state.filter_house,
                                        h,
                                        h.to_string(),
                                    );
                                }
                            });
                        ui.add(egui::Slider::new(&mut self.state.filter_floor, 0..=9).text("Hæð"));
                        ui.add(
                            egui::Slider::new(&mut self.state.filter_room, 1..=999)
                                .text("Herbergi"),
                        );
                    }
                    PrintMode::ByFloor => {
                        egui::ComboBox::from_label("Hús")
                            .selected_text(self.state.filter_house.to_string())
                            .show_ui(ui, |ui| {
                                for h in [House::HA, House::H, House::S] {
                                    ui.selectable_value(
                                        &mut self.state.filter_house,
                                        h,
                                        h.to_string(),
                                    );
                                }
                            });
                        ui.add(egui::Slider::new(&mut self.state.filter_floor, 0..=9).text("Hæð"));
                    }
                }

                if ui.button("Prenta á skjá").clicked() {
                    let result = match self.state.print_mode {
                        PrintMode::All => self
                            .inv
                            .all()
                            .map(|items| ("Allur búnaður".to_string(), items)),
                        PrintMode::ByHouse => {
                            let house = self.state.filter_house;
                            self.inv
                                .by_house(house)
                                .map(|items| (format!("Búnaður í húsi {}", house), items))
                        }
                        PrintMode::ByKind => {
                            let kind = self.state.filter_kind;
                            self.inv.by_kind(kind).map(|items| {
                                (format!("Búnaður af tegund {}", kind.friendly_name()), items)
                            })
                        }
                        PrintMode::ByRoom => {
                            let loc = Location {
                                house: self.state.filter_house,
                                floor: self.state.filter_floor,
                                room: self.state.filter_room,
                            };
                            self.inv
                                .by_room(&loc)
                                .map(|items| (format!("Búnaður í stofu {}", loc), items))
                        }
                        PrintMode::ByFloor => {
                            let house = self.state.filter_house;
                            let floor = self.state.filter_floor;
                            self.inv.by_floor(house, floor).map(|items| {
                                (format!("Búnaður á {}. hæð í {}-húsi", floor, house), items)
                            })
                        }
                    };

                    match result {
                        Ok((title, records)) => {
                            let count = records.len();
                            self.set_print_data(title, records);
                            let ending = if count == 1 { "" } else { "ir" };
                            self.status = Some(format!("Prentaði {} hlut{}.", count, ending));
                        }
                        Err(e) => {
                            self.status = Some(format!("Villa við prentun: {}", e));
                            self.print_results.clear();
                            self.print_title = None;
                        }
                    }
                }

                ui.separator();
                ui.heading("Aðgerðir eftir ID");
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.state.print_id).hint_text("ID"));
                    if ui.button("Sýna").clicked() {
                        match self.state.print_id.trim().parse::<i64>() {
                            Ok(id) if id > 0 => match self.inv.by_id(id) {
                                Ok(Some(rec)) => {
                                    self.set_print_data(
                                        format!("Búnaður með ID {}", id),
                                        vec![rec],
                                    );
                                    self.status = Some(format!("Fundum búnað með ID {}.", id));
                                }
                                Ok(None) => {
                                    self.status = Some("Ekkert fannst með þetta ID.".into())
                                }
                                Err(e) => self.status = Some(format!("Villa: {}", e)),
                            },
                            _ => self.status = Some("ID þarf að vera jákvæð tala.".into()),
                        }
                    }
                    if ui.button("Eyða").clicked() {
                        match self.state.print_id.trim().parse::<i64>() {
                            Ok(id) if id > 0 => match self.inv.remove(id) {
                                Ok(true) => {
                                    self.status = Some("Eytt.".into());
                                    self.reload_items();
                                }
                                Ok(false) => {
                                    self.status = Some("Ekkert fannst með þetta ID.".into())
                                }
                                Err(e) => self.status = Some(format!("Villa: {}", e)),
                            },
                            _ => self.status = Some("ID þarf að vera jákvæð tala.".into()),
                        }
                    }
                });

                if let Some(title) = &self.print_title {
                    ui.separator();
                    ui.heading("Prentniðurstöður");
                    ui.label(egui::RichText::new(title).strong());
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for line in &self.print_results {
                                ui.label(line);
                            }
                        });
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Listi");
            ui.separator();

            let text_height = egui::TextStyle::Body.resolve(ui.style()).size * 1.2;
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .column(egui_extras::Column::exact(60.0))
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::remainder())
                    .header(text_height, |mut row| {
                        row.col(|ui| {
                            ui.strong("Id");
                        });
                        row.col(|ui| {
                            ui.strong("Tegund");
                        });
                        row.col(|ui| {
                            ui.strong("Staðsetning");
                        });
                        row.col(|ui| {
                            ui.strong("Verðmæti");
                        });
                        row.col(|ui| {
                            ui.strong("Auka-upplýsingar");
                        });
                    })
                    .body(|mut body| {
                        for item in &self.items {
                            body.row(text_height, |mut row| {
                                row.col(|ui| {
                                    ui.monospace(item.id.unwrap_or_default().to_string());
                                });
                                row.col(|ui| {
                                    ui.label(item.kind.friendly_name());
                                });
                                row.col(|ui| {
                                    ui.label(item.location.to_string());
                                });
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{} kr.",
                                        item.value_isk.separate_with_spaces()
                                    ));
                                });
                                row.col(|ui| {
                                    match item.kind {
                                        EquipmentKind::Table => ui.label(format!(
                                            "Sæti: {}",
                                            item.seats.unwrap_or_default()
                                        )),
                                        EquipmentKind::Chair => ui.label(format!(
                                            "Gerð: {}",
                                            item.chair_kind
                                                .map(|c| c.to_string())
                                                .unwrap_or_else(|| "-".into())
                                        )),
                                        EquipmentKind::Projector => ui.label(format!(
                                            "Lumens: {}",
                                            item.lumens.unwrap_or_default()
                                        )),
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
            update_house: House::H,
            update_floor: 1,
            update_room: 1,
            update_id: String::new(),
            filter_house: House::H,
            filter_floor: 1,
            filter_room: 1,
            filter_kind: EquipmentKind::Table,
            print_mode: PrintMode::All,
            print_id: String::new(),
        }
    }
}
