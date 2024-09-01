use iced::{
    alignment,
    widget::{button, column, container, row, scrollable, svg, text, text_input, Column},
    Element, Length, Padding, Sandbox, Theme,
};

pub struct Index {
    input_pulls: u32,
    input_pity_character: usize,
    input_capture_radiance: u32,
    input_focus_character: bool,
    input_pity_weapon: usize,
    input_epitomized_path: bool,
    input_focus_weapon: bool,
    input_constellation: i32,
    input_refinement: u32,
    wanted_constellation: i32,
    wanted_refinement: u32,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            input_pulls: 0,
            input_pity_character: 0,
            input_capture_radiance: 0,
            input_focus_character: false,
            input_pity_weapon: 0,
            input_epitomized_path: false,
            input_focus_weapon: false,
            input_constellation: -1,
            input_refinement: 0,
            wanted_constellation: 0,
            wanted_refinement: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Amount {
    InputValue(usize),
}

impl Sandbox for Index {
    type Message = Amount;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "index".into()
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        container(text(""))
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
