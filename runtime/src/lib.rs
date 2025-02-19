mod render;
mod fps_printer;
mod compiler;

use std::time::Duration;

use render::{Color, Scene};
use wasm_bindgen::prelude::*;
use js_sys::Uint8ClampedArray;
use fps_printer::FpsPrinter;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static FPS_PRINTER: FpsPrinter = FpsPrinter::new();

static mut SCENE : Option<Scene> = None;

#[wasm_bindgen]
pub fn init() -> Uint8ClampedArray  {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("init");

    FPS_PRINTER.init();

    unsafe {
        SCENE = Some(Scene::new());
    }
    
    unsafe { 
        Uint8ClampedArray::view(render::frame::raw_buffer())
    }
}

#[wasm_bindgen]
pub fn compile(input: &str) -> Result<String, JsError> {
    compiler::compile(input).map_err(|e| JsError::from(&*e))
}

#[wasm_bindgen]
pub fn render() {
    FPS_PRINTER.tick();

    let scene = unsafe { SCENE.as_mut().unwrap() };
    do_scene(scene);
    scene.render();
}

fn do_scene(scene: &mut Scene) {
    const BLUE: Color = Color::from_rgb(0, 0, 255);

    static mut LAST_TIME: wasm_timer::SystemTime = wasm_timer::SystemTime::UNIX_EPOCH;
    static mut LAST_INDEX: usize = 0;
    static mut INITIALIZED: bool = false;

    let last_time = unsafe { &mut LAST_TIME };
    let last_index = unsafe { &mut LAST_INDEX };
    let initialized = unsafe { &mut INITIALIZED };

    if !*initialized {
        *initialized = true;

        for i in 0..Scene::LIGHT_COUNT {
            scene.set_light_color(i, BLUE);
        }
    }

    let now = wasm_timer::SystemTime::now();
    if now > last_time.checked_add(Duration::from_millis(20)).unwrap() {
        *last_time = now;

       
        scene.set_light_color(*last_index, BLUE);
        *last_index = (*last_index + 1) % Scene::LIGHT_COUNT;
        scene.set_light_color(*last_index, Color::WHITE);
    }
}