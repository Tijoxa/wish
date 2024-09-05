use backend::simulate_n;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

pub struct Index {
    input_pulls: u32,
    cashback: bool,
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
            cashback: false,
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
    PullsChangedCashback(bool),
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

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "gi_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../resources/zh-cn.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "gi_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("gi_font".to_owned());
    ctx.set_fonts(fonts);
}

impl Index {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for Index {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wish Planner");

            ui.add(egui::Image::new(egui::include_image!(
                "../resources/chiori_doll.svg"
            )))
        });
    }
}
/*
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
            Message::PullsChangedCashback(v) => {
                match v {
                    true => self.input_pulls = (1.1 * self.input_pulls as f32) as u32,
                    false => self.input_pulls = (self.input_pulls as f32 / 1.1) as u32,
                };
                self.cashback = v;
            }
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
            container(
                text("Wish Planner")
                    .font(Font {
                        weight: font::Weight::ExtraBold,
                        ..Default::default()
                    })
                    .size(40)
            )
            .padding(5),
            svg(handle.clone())
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0)),
        ));

        let container_top: container::Container<'_, Message, Theme, Renderer> = container(row!(
            container(column!(
                container(text(format!("Available wishes: {}", self.input_pulls))).padding(5),
                container(slider(0..=1000, self.input_pulls, Message::PullsChanged)).padding(5),
            )),
            container(
                checkbox("Include 10% cashback", self.cashback)
                    .on_toggle(Message::PullsChangedCashback)
            )
            .padding(5),
        ));

        let container_left: container::Container<'_, Message, Theme, Renderer> =
            container(column!(
                container(text("=== CHARACTER ===").font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                }))
                .padding(7),
                container(text(format!(
                    "Current character banner pity: {}",
                    self.input_pity_character
                )))
                .padding(5),
                container(slider(
                    0..=89,
                    self.input_pity_character as u32,
                    Message::PityCharacterChanged
                ))
                .padding(5),
                container(text(format!(
                    "Current capturing radiance: {}",
                    self.input_capture_radiance
                )))
                .padding(5),
                container(slider(
                    1..=4,
                    self.input_capture_radiance,
                    Message::CaptureRadianceChanged
                ))
                .padding(5),
                container(
                    checkbox("Guaranteed character", self.input_focus_character)
                        .on_toggle(Message::FocusCharacterChanged)
                )
                .padding(5),
                container(text("=== WEAPON ===").font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                }))
                .padding(7),
                container(text(format!(
                    "Current weapon banner pity: {}",
                    self.input_pity_weapon
                )))
                .padding(5),
                container(slider(
                    0..=76,
                    self.input_pity_weapon as u32,
                    Message::PityWeaponChanged
                ))
                .padding(5),
                container(
                    checkbox("Epitomized path", self.input_epitomized_path)
                        .on_toggle(Message::EpitomizedPathChanged)
                )
                .padding(5),
                container(
                    checkbox("Garanteed focus weapon", self.input_focus_weapon)
                        .on_toggle(Message::FocusWeaponChanged)
                )
                .padding(5),
            ));

        let container_right: container::Container<'_, Message, Theme, Renderer> =
            container(column!(
                container(text(match self.input_constellation {
                    -1 => "Current constellation: None".to_string(),
                    _ => {
                        format!("Current constellation: {}", self.input_constellation)
                    }
                }))
                .padding(5),
                container(slider(
                    -1..=6,
                    self.input_constellation,
                    Message::CurrentConstellationChanged
                ))
                .padding(5),
                container(text(format!(
                    "Current refinement: {}",
                    self.input_refinement
                )))
                .padding(5),
                container(slider(
                    0..=5,
                    self.input_refinement,
                    Message::CurrentRefinementChanged
                ))
                .padding(5),
                container(text(match self.wanted_constellation {
                    -1 => "Wanted constellation: None".to_string(),
                    _ => {
                        format!("Wanted constellation: {}", self.wanted_constellation)
                    }
                }))
                .padding(5),
                container(slider(
                    -1..=6,
                    self.wanted_constellation,
                    Message::WantedConstellationChanged
                ))
                .padding(5),
                container(text(format!(
                    "Wanted refinement: {}",
                    self.wanted_refinement
                )))
                .padding(5),
                container(slider(
                    0..=5,
                    self.wanted_refinement,
                    Message::WantedRefinementChanged
                ))
                .padding(5),
            ));

        let container_bottom: container::Container<'_, Message, Theme, Renderer> = container(row!(
            container(button("Submit").on_press(Message::Simulate)).padding(5),
            container(
                text(format!(
                    "Estimated probability: {}",
                    self.estimated_probability
                ))
                .size(20)
                .font(Font {
                    weight: font::Weight::Semibold,
                    ..Default::default()
                })
            )
            .padding(5)
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
*/
