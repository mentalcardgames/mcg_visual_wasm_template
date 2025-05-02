mod utils;

use eframe::CreationContext;
use egui_extras::install_image_loaders;
use mcg_visual::game::App;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Promise;
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    pub fn openDirectoryPicker() -> Promise;
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let init = Box::new(|cc: &CreationContext| {
        install_image_loaders(&cc.egui_ctx);
        let app = App::new();
        let game: Box<dyn eframe::App> = Box::new(app);
        Ok(game)
    });
    mcg_visual::start_game(canvas, init)
}
