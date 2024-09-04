#![windows_subsystem = "windows"]
use iced::{
    window::{self, icon},
    Application, Settings, Size,
};
use wish::index::Index;

fn main() -> iced::Result {
    let icon = icon::from_file_data(include_bytes!("../resources/genesis_crystal.png"), None)
        .expect("Failed to create icon");
    let settings = Settings {
        window: window::Settings {
            size: Size::new(800., 600.),
            min_size: Some(Size::new(600., 600.)),
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    };
    Index::run(settings)
}
