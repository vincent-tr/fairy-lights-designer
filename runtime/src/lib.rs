mod render;
mod fps_printer;
mod compiler;
mod vm;

use render::{Color, Scene};
use vm::executable::Executable;
use wasm_bindgen::prelude::*;
use js_sys::{Math, Uint8ClampedArray};
use fps_printer::FpsPrinter;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static FPS_PRINTER: FpsPrinter = FpsPrinter::new();

static mut SCENE : Option<Scene> = None;

static mut VM: Option<vm::VM> = None;

struct VMApi;

impl vm::ExternalApi for VMApi {
    fn rand(&self, min: i32, max: i32) -> i32 {
        let value = Math::random() * (max - min) as f64 + min as f64;
        value.round() as i32
    }

    fn len(&self) -> usize {
        Scene::LIGHT_COUNT
    }

    fn get(&self, index: usize) -> (u8, u8, u8) {
        let scene = unsafe { SCENE.as_ref().unwrap() };
        let color = scene.get_light_color(index);
        (color.red(), color.green(), color.blue())
    }

    fn set(&self, index: usize, color: (u8, u8, u8)) {
        let scene = unsafe { SCENE.as_mut().unwrap() };
        let color = Color::from_rgb(color.0, color.1, color.2);
        scene.set_light_color(index, color);
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("init");

    FPS_PRINTER.init();

    unsafe {
        SCENE = Some(Scene::new());
    }

    unsafe {
        VM = Some(vm::VM::new(Box::new(VMApi)));
    }
}

#[wasm_bindgen]
pub fn compile(input: &str) -> Result<String, JsError> {
    compiler::compile(input).map_err(|e| JsError::from(&*e))
}

#[wasm_bindgen]
pub fn execute(input: &str) -> Result<(), JsError> {
    let exec = Executable::from_text(input).map_err(|e| JsError::from(&*e))?;

    let vm = unsafe { VM.as_mut().unwrap() };
    vm.load_executable(exec);

    Ok(())
}

#[wasm_bindgen]
pub fn render() -> Uint8ClampedArray {
    FPS_PRINTER.tick();

    let vm = unsafe { VM.as_mut().unwrap() };
    vm.tick();

    let scene = unsafe { SCENE.as_mut().unwrap() };
    // do_scene(scene);
    scene.render();

    
    unsafe { 
        Uint8ClampedArray::view(render::frame::raw_buffer())
    }
}
/*
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
*/