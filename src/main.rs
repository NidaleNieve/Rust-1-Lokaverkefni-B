mod app;
mod db;
mod inventory;
mod models;

use dotenvy::dotenv;
use eframe::NativeOptions;
use std::env;

fn main() -> eframe::Result<()> {
    dotenv().ok();

    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "app_data/inventory.sqlite3".to_string());
    std::fs::create_dir_all(std::path::Path::new(&db_path).parent().unwrap()).ok();

    let inv = inventory::Inventory::open(&db_path).expect("Gat ekki opnað gagnagrunn");

    let mut native_opts = NativeOptions::default();
    native_opts.viewport = egui::ViewportBuilder::default()
        .with_inner_size([1080.0, 720.0])
        .with_min_inner_size([900.0, 600.0])
        .with_title("Búnaðarlisti Tækniskólans");

    eframe::run_native(
        "Búnaðarlisti Tækniskólans",
        native_opts,
        Box::new(move |_cc| Box::new(app::BunadarApp::new(inv))),
    )
}
