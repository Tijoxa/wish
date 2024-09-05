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

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
            ui.add_space(20.);
            ui.horizontal(|ui| {
                ui.columns(2, |c| {
                    c[0].horizontal(|cc| {
                        cc.add_space(ctx.available_rect().width() * 0.1);
                        if cc.add_sized([40., 40.], egui::Button::new("-10")).clicked() {
                            self.input_pulls = (self.input_pulls - 10).max(0);
                        }
                        if cc.add_sized([30., 30.], egui::Button::new("-1")).clicked() {
                            self.input_pulls = (self.input_pulls - 1).max(0);
                        }
                        cc.horizontal(|cc| {
                            cc.set_width(160.);
                            cc.vertical_centered(|cc| {
                                cc.add(
                                    egui::DragValue::new(&mut self.input_pulls)
                                        .range(0..=9999)
                                        .speed(0.7)
                                        .prefix("Available wishes: "),
                                );
                            });
                        });
                        if cc.add_sized([30., 30.], egui::Button::new("+1")).clicked() {
                            self.input_pulls = (self.input_pulls + 1).min(u32::MAX);
                        }
                        if cc.add_sized([40., 40.], egui::Button::new("+10")).clicked() {
                            self.input_pulls = (self.input_pulls + 10).min(u32::MAX);
                        }
                    });

                    if c[1]
                        .add_sized(
                            [40., 40.],
                            egui::Checkbox::new(&mut self.cashback, "Include 10% cashback"),
                        )
                        .changed()
                    {
                        match &self.cashback {
                            true => self.input_pulls = (self.input_pulls as f64 * 1.1) as u32,
                            false => self.input_pulls = (self.input_pulls as f64 / 1.1) as u32,
                        }
                    }
                });
            });
            ui.add_space(20.);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(ctx.available_rect().width() * 0.5 - 370.);
                ui.vertical(|ui| {
                    ui.add_space(ctx.available_rect().height() * 0.5 - 170.);
                    ui.columns(2, |c| {
                        // First column
                        // Character
                        c[0].vertical_centered(|cc| {
                            cc.label("Character");
                        });
                        c[0].add_space(5.);
                        c[0].horizontal(|cc| {
                            cc.label("Current character banner pity");
                            cc.add(
                                egui::DragValue::new(&mut self.input_pity_character)
                                    .range(0..=89)
                                    .speed(0.7)
                                    .prefix("Value: "),
                            );
                        });
                        c[0].horizontal(|cc| {
                            cc.label("Current capturing radiance");
                            cc.add(egui::Slider::new(&mut self.input_capture_radiance, 1..=4));
                        });
                        c[0].horizontal(|cc| {
                            cc.label("Guaranteed character");
                            cc.checkbox(&mut self.input_focus_character, "");
                        });
                        // Weapon
                        c[0].add_space(10.);
                        c[0].vertical_centered(|cc| {
                            cc.label("Weapon");
                        });
                        c[0].add_space(5.);
                        c[0].horizontal(|cc| {
                            cc.label("Current weapon banner pity");
                            cc.add(
                                egui::DragValue::new(&mut self.input_pity_weapon)
                                    .range(0..=76)
                                    .speed(0.7)
                                    .prefix("Value: "),
                            );
                        });
                        c[0].horizontal(|cc| {
                            cc.label("Epitomized path");
                            cc.checkbox(&mut self.input_epitomized_path, "");
                        });
                        c[0].horizontal(|cc| {
                            cc.label("Guaranteed weapon");
                            cc.checkbox(&mut self.input_focus_weapon, "");
                        });
                        // Second column
                        // Current
                        c[1].add_space(30.);
                        c[1].horizontal(|cc| {
                            cc.columns(2, |ccc| {
                                ccc[0].label("Current constellation");
                                ccc[1]
                                    .add(egui::Slider::new(&mut self.input_constellation, -1..=6));
                            });
                        });
                        c[1].add_space(10.);
                        c[1].horizontal(|cc| {
                            cc.columns(2, |ccc| {
                                ccc[0].label("Current refinement");
                                ccc[1].add(egui::Slider::new(&mut self.input_refinement, 0..=5));
                            });
                        });
                        // Wanted
                        c[1].add_space(20.);
                        c[1].horizontal(|cc| {
                            cc.columns(2, |ccc| {
                                ccc[0].label("Wanted constellation");
                                ccc[1]
                                    .add(egui::Slider::new(&mut self.wanted_constellation, -1..=6));
                            });
                        });
                        c[1].add_space(10.);
                        c[1].horizontal(|cc| {
                            cc.columns(2, |ccc| {
                                ccc[0].label("Wanted refinement");
                                ccc[1].add(egui::Slider::new(&mut self.wanted_refinement, 0..=5));
                            });
                        });
                    });
                });
            });
        });

        egui::TopBottomPanel::bottom("simulation").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.);
                if ui
                    .add_sized([200., 50.], egui::Button::new("Submit"))
                    .clicked()
                {
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
                ui.add_space(20.);
                ui.label(match self.estimated_probability {
                    Some(r) => format!("Probability: {:.2}%", 100. * r),
                    None => "Probability: None".to_string(),
                });
                ui.add_space(20.);
            });
        });
    }
}
