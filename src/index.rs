use backend::simulate_n;
use eframe::egui;

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
    estimated_probability: Option<f64>,
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
            estimated_probability: None,
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
        Self::default()
    }
}

impl eframe::App for Index {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("title").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.columns(3, |c| {
                    let image = egui::include_image!("../resources/chiori_doll.svg");
                    c[0].vertical_centered(|cc| {
                        cc.add(egui::Image::new(image.clone()).fit_to_original_size(0.4));
                    });
                    c[1].with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |cc| {
                            cc.heading(egui::RichText::new("Wish Planner").size(32.0));
                        },
                    );
                    c[2].vertical_centered(|cc| {
                        cc.add(egui::Image::new(image).fit_to_original_size(0.4));
                    });
                });
            });
        });

        egui::TopBottomPanel::top("wishes").show(ctx, |ui| {
            ui.label("Available wishes");
            ui.label("Include 10% cashback");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |c| {
                c[0].vertical_centered(|cc| {
                    cc.label("Character");
                });
                c[0].label("Current character banner pity");
                c[0].label("Current capturing radiance");
                c[0].label("Guaranteed character");
                c[0].vertical_centered(|cc| {
                    cc.label("Weapon");
                });
                c[0].label("Current weapon banner pity");
                c[0].label("Epitomized path");
                c[0].label("Guaranteed character");

                c[1].label("Current constellation");
                c[1].label("Current refinement");
                c[1].label("Wanted constellation");
                c[1].label("Wanted refinement");
            })
        });

        egui::TopBottomPanel::bottom("simulation").show(ctx, |ui| {
            if ui.button("Submit").clicked() {
                self.estimated_probability = Some(simulate_n(
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
                ));
            };
            ui.label(match self.estimated_probability {
                Some(r) => format!("Probability: {}", r),
                None => "Probability:".to_string(),
            })
        });
    }
}
