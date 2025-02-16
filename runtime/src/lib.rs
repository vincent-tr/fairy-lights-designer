mod render;
mod fps_printer;

use render::{drawing::{Circle, Color, Drawable, Fillable, Line, Point, Rectangle, Size}, scene::Scene};
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
pub fn render() {
    FPS_PRINTER.tick();

    unsafe {
        let scene = SCENE.as_mut().unwrap();

        for i in 0..100 {
            scene.set_light_color(i, Color::from_rgb((50 + i * 2) as u8, 0, if i % 2 == 0 { 0 } else { 255 }));
        }

        scene.render();
    }
}

