#![windows_subsystem = "windows"]
use eframe::egui;
use wish::index::Index;

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800., 520.])
            .with_min_inner_size([800., 520.])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../resources/intertwined_fate.png")[..],
                )
                .expect("Failed to load icon"),
            )
            .with_position([200., 50.]),
        ..Default::default()
    };
    eframe::run_native(
        "Wish Planner",
        native_options,
        Box::new(|cc| Ok(Box::new(Index::new(cc)))),
    )
}
