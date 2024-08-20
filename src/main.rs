use iced::{Application, Settings};
use wish::Wish;

fn main() -> iced::Result {
    Wish::run(Settings::default())
}
