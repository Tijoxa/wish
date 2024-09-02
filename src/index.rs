use backend::simulate_n;
use iced::{
    alignment,
    widget::{
        button, checkbox, column, container, row, scrollable, slider, svg, text, text_input,
        Column, Space,
    },
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
    estimated_probability: f64,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            input_pulls: 0,
            input_pity_character: 0,
            input_capture_radiance: 1,
            input_focus_character: false,
            input_pity_weapon: 0,
            input_epitomized_path: false,
            input_focus_weapon: false,
            input_constellation: -1,
            input_refinement: 0,
            wanted_constellation: 0,
            wanted_refinement: 1,
            estimated_probability: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PullsChanged(u32),
    // Character
    PityCharacterChanged(u32),
    CaptureRadianceChanged(u32),
    FocusCharacterChanged(bool),
    // Weapon
    PityWeaponChanged(u32),
    EpitomizedPathChanged(bool),
    FocusWeaponChanged(bool),
    // Current
    CurrentConstellationChanged(i32),
    CurrentRefinementChanged(u32),
    // Wanted
    WantedConstellationChanged(i32),
    WantedRefinementChanged(u32),
    // Simulate
    Simulate,
}

impl Sandbox for Index {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "index".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PullsChanged(v) => self.input_pulls = v,
            // Character
            Message::PityCharacterChanged(v) => self.input_pity_character = v as usize,
            Message::CaptureRadianceChanged(v) => self.input_capture_radiance = v,
            Message::FocusCharacterChanged(v) => self.input_focus_character = v,
            // Weapon
            Message::PityWeaponChanged(v) => self.input_pity_weapon = v as usize,
            Message::EpitomizedPathChanged(v) => self.input_epitomized_path = v,
            Message::FocusWeaponChanged(v) => self.input_focus_weapon = v,
            // Current
            Message::CurrentConstellationChanged(v) => self.input_constellation = v,
            Message::CurrentRefinementChanged(v) => self.input_refinement = v,
            // Wanted
            Message::WantedConstellationChanged(v) => self.wanted_constellation = v,
            Message::WantedRefinementChanged(v) => self.wanted_refinement = v,
            // Simulate
            Message::Simulate => {
                self.estimated_probability = simulate_n(
                    self.input_pulls,
                    self.input_pity_character,
                    self.input_capture_radiance,
                    self.input_focus_character,
                    self.input_pity_weapon,
                    self.input_epitomized_path,
                    self.input_focus_weapon,
                    self.input_constellation,
                    self.input_refinement,
                    self.wanted_constellation,
                    self.wanted_refinement,
                );
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        container(column!(
            row!(
                text("nb pulls: "),
                slider(0..=1000, self.input_pulls, Message::PullsChanged),
                text(format!(" Current value: {}", self.input_pulls)),
            ),
            column!(
                text("\n=== CHARACTER ==="),
                text(""),
                row!(
                    text("pity character: "),
                    slider(
                        0..=89,
                        self.input_pity_character as u32,
                        Message::PityCharacterChanged
                    ),
                    text(format!(" Current value: {}", self.input_pity_character)),
                ),
                row!(
                    text("capture radiance: "),
                    slider(
                        1..=4,
                        self.input_capture_radiance,
                        Message::CaptureRadianceChanged
                    ),
                    text(format!(" Current value: {}", self.input_capture_radiance)),
                ),
                row!(checkbox("garanteed character", self.input_focus_character)
                    .on_toggle(Message::FocusCharacterChanged)),
                text("\n=== WEAPON ==="),
                text(""),
                row!(
                    text("pity weapon: "),
                    slider(
                        0..=76,
                        self.input_pity_weapon as u32,
                        Message::PityWeaponChanged
                    ),
                    text(format!(" Current value: {}", self.input_pity_weapon)),
                ),
                row!(checkbox("epitomized path", self.input_epitomized_path)
                    .on_toggle(Message::EpitomizedPathChanged)),
                row!(checkbox("garanteed focus weapon", self.input_focus_weapon)
                    .on_toggle(Message::FocusWeaponChanged)),
            ),
            column!(
                // vertical line
            ),
            column!(
                text("Current constellation"),
                slider(
                    -1..=6,
                    self.input_constellation,
                    Message::CurrentConstellationChanged
                ),
                text("Current refinement"),
                slider(
                    0..=5,
                    self.input_refinement,
                    Message::CurrentRefinementChanged
                ),
                text("Wanted constellation"),
                slider(
                    -1..=6,
                    self.wanted_constellation,
                    Message::WantedConstellationChanged
                ),
                text("Wanted refinement"),
                slider(
                    0..=5,
                    self.wanted_refinement,
                    Message::WantedRefinementChanged
                ),
            ),
            row!(
                button("Submit").on_press(Message::Simulate),
                text(format!(
                    "Estimated probability: {}",
                    self.estimated_probability
                ))
            )
        ))
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
