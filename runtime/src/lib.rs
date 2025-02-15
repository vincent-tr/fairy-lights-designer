use wasm_bindgen::prelude::*;
use js_sys::Uint8ClampedArray;
use std::slice;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn init() -> Uint8ClampedArray  {
    console_error_panic_hook::set_once();

    console_log!("init");
    
    unsafe { 
        let raw_buffer = slice::from_raw_parts(&raw const BUFFER as *const FrameBuffer as *const u8, std::mem::size_of::<FrameBuffer>());
        Uint8ClampedArray::view(&raw_buffer)
    }
}

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[repr(C)]
#[derive(Debug)]
struct FrameBuffer {
    pixels: [Pixel; WIDTH * HEIGHT],
}

static mut BUFFER: FrameBuffer = FrameBuffer {
    pixels: [Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }; WIDTH * HEIGHT],
};

#[wasm_bindgen]
pub fn render() {

    for i in 0..(WIDTH*HEIGHT) {
        unsafe {
            BUFFER.pixels[i] = Pixel {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            };
        }
    }
}
