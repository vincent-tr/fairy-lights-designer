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
    render::drawing::clear(Color::from_rgb(0, 0, 0));

    Rectangle::new(Point::new(800, 10), Size::new(10, 10)).fill(Color::from_rgb(0, 0, 255));
    Circle::new(Point::new(800, 200), 100).fill(Color::from_rgb(0, 0, 255));

    Rectangle::new(Point::new(100, 10), Size::new(20, 10)).draw(Color::from_rgb(0, 255, 0));
    Line::new(Point::new(10, 10), Point::new(40, 20)).draw(Color::from_rgb(0, 255, 0));
    Circle::new(Point::new(20, 20), 5).draw(Color::from_rgb(0, 255, 0));
    
    FPS_PRINTER.tick();

    unsafe {
        let scene = SCENE.as_mut().unwrap();

        for i in 0..100 {
            scene.set_light_color(i, Color::from_rgb((50 + i * 2) as u8, 0, if i % 2 == 0 { 0 } else { 255 }));
        }

        scene.render();
    }
}

