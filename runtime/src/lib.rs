mod render;
mod fps_printer;

use render::drawing::{Point, Size, Rectangle, Color};
use wasm_bindgen::prelude::*;
use js_sys::Uint8ClampedArray;
use fps_printer::FpsPrinter;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static FPS_PRINTER: FpsPrinter = FpsPrinter::new();

#[wasm_bindgen]
pub fn init() -> Uint8ClampedArray  {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("init");

    FPS_PRINTER.init();
    
    unsafe { 
        Uint8ClampedArray::view(render::frame::raw_buffer())
    }
}

#[wasm_bindgen]
pub fn render() {
    render::drawing::clear(Color::from_rgb(255, 0, 0));

    render::drawing::draw_rect(&Rectangle::new(Point::new(800, 10), Size::new(10, 10)) , Color::from_rgb(0, 0, 255));

    
    FPS_PRINTER.tick();
}

