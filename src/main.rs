use iced::{
    window::{self, icon},
    Application, Settings, Size,
};
use wish::Wish;

fn main() -> iced::Result {
    let icon = icon::from_file_data(
        include_bytes!("../resources/genesis_crystal.png").as_slice(),
        None,
    )
    .expect("Failed to create icon");
    let settings = Settings {
        window: window::Settings {
            size: Size::new(800., 600.),
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    };
    Wish::run(settings)
}
