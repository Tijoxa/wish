use backend::simulate_n;
use iced::{
    alignment, font,
    widget::{button, checkbox, column, container, row, slider, svg, text, text_input},
    Element, Font, Length, Renderer, Sandbox, Theme,
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
        String::from("Wish Planner")
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
        let handle =
            svg::Handle::from_memory(include_bytes!("../resources/chiori_doll.svg").as_slice());
        let container_title: container::Container<'_, Message, Theme, Renderer> = container(row!(
            svg(handle.clone())
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0)),
            text("Wish Planner")
                .font(Font {
                    weight: font::Weight::ExtraBold,
                    ..Default::default()
                })
                .size(40),
            svg(handle.clone())
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0)),
        ));

        let container_top: container::Container<'_, Message, Theme, Renderer> = container(column!(
            text(format!("Available wishes: {}", self.input_pulls)),
            slider(0..=1000, self.input_pulls, Message::PullsChanged),
        ));

        let container_left: container::Container<'_, Message, Theme, Renderer> =
            container(column!(
                container(text("=== CHARACTER ===").font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                }))
                .padding(5),
                text(format!(
                    "Current character banner pity: {}",
                    self.input_pity_character
                )),
                slider(
                    0..=89,
                    self.input_pity_character as u32,
                    Message::PityCharacterChanged
                ),
                text(format!(
                    "Current capturing radiance: {}",
                    self.input_capture_radiance
                )),
                slider(
                    1..=4,
                    self.input_capture_radiance,
                    Message::CaptureRadianceChanged
                ),
                checkbox("Garanteed character", self.input_focus_character)
                    .on_toggle(Message::FocusCharacterChanged),
                container(text("=== WEAPON ===").font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                }))
                .padding(5),
                text(format!(
                    "Current weapon banner pity: {}",
                    self.input_pity_weapon
                )),
                slider(
                    0..=76,
                    self.input_pity_weapon as u32,
                    Message::PityWeaponChanged
                ),
                checkbox("Epitomized path", self.input_epitomized_path)
                    .on_toggle(Message::EpitomizedPathChanged),
                checkbox("Garanteed focus weapon", self.input_focus_weapon)
                    .on_toggle(Message::FocusWeaponChanged),
            ));

        let container_right: container::Container<'_, Message, Theme, Renderer> =
            container(column!(
                text(match self.input_constellation {
                    -1 => "Current constellation: None".to_string(),
                    _ => {
                        format!("Current constellation: {}", self.input_constellation)
                    }
                }),
                slider(
                    -1..=6,
                    self.input_constellation,
                    Message::CurrentConstellationChanged
                ),
                text(format!("Current refinement: {}", self.input_refinement)),
                slider(
                    0..=5,
                    self.input_refinement,
                    Message::CurrentRefinementChanged
                ),
                text(match self.wanted_constellation {
                    -1 => "Wanted constellation: None".to_string(),
                    _ => {
                        format!("Wanted constellation: {}", self.wanted_constellation)
                    }
                }),
                slider(
                    -1..=6,
                    self.wanted_constellation,
                    Message::WantedConstellationChanged
                ),
                text(format!("Wanted refinement: {}", self.wanted_refinement)),
                slider(
                    0..=5,
                    self.wanted_refinement,
                    Message::WantedRefinementChanged
                ),
            ));

        let container_bottom: container::Container<'_, Message, Theme, Renderer> =
            container(column!(
                button("Submit").on_press(Message::Simulate),
                text(format!(
                    "Estimated probability: {}",
                    self.estimated_probability
                ))
                .size(20)
                .font(Font {
                    weight: font::Weight::Semibold,
                    ..Default::default()
                })
            ));

        let content = column!(
            container_title.padding(5),
            container_top.padding(5),
            container(row!(container_left.padding(5), container_right.padding(5))),
            container_bottom.padding(5)
        );

        container(content).padding(20).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
