#![cfg_attr(not(target_arch = "wasm32"), windows_subsystem = "windows")]
#[cfg(not(target_arch = "wasm32"))]
use eframe::egui;
use wish_lib::index::Index;

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(Index::new(cc)))),
            )
            .await;

        if let Err(e) = start_result {
            log::error!("Failed to start eframe: {e:?}");
        }
    });
}
