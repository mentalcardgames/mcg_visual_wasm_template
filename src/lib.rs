mod utils;

use eframe::CreationContext;
use egui_extras::install_image_loaders;
use mcg_visual::game::screen::{Game, GameSetupScreen};
use mcg_visual::game::App;
use std::cell::RefCell;
use std::rc::Rc;
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
        let mut app = App::default();
        let game_widget = Rc::new(RefCell::new(Game::new()));
        let game_conf = Rc::new(RefCell::new(GameSetupScreen::new(Rc::downgrade(
            &game_widget,
        ))));
        app.register_screen(String::from("game"), game_widget)
            .unwrap();
        app.register_screen(String::from("game_setup"), game_conf)
            .unwrap();
        let game: Box<dyn eframe::App> = Box::new(app);
        Ok(game)
    });
    mcg_visual::start_game(canvas, init)
}
