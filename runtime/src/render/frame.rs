pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 1000;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const BLACK: Self = Self::from_rgb(0,0,0);
    pub const WHITE: Self = Self::from_rgb(255,255,255);

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn red(&self) -> u8 {
        self.r
    }

    pub const fn green(&self) -> u8 {
        self.g
    }

    pub const fn blue(&self) -> u8 {
        self.b
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FrameBuffer {
    pixels: [Color; WIDTH * HEIGHT],
}

impl FrameBuffer {
    #[allow(dead_code)]
    pub fn pixel(&self, x: usize, y: usize) -> &Color {
        &self.pixels[self.index(x, y)]
    }

    #[allow(dead_code)]
    pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.pixels[self.index(x, y)]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[self.index(x, y)] = color;
    }

    fn index(&self, x: usize, y: usize) -> usize {
        if x >= WIDTH || y >= HEIGHT {
            panic!("Out of bounds access: x={}, y={}", x, y);
        }

        y * WIDTH + x
    }
}

static mut BUFFER: FrameBuffer = FrameBuffer {
  pixels: [Color {
      r: 0,
      g: 0,
      b: 0,
      a: 255,
  }; WIDTH * HEIGHT],
};

pub fn frame() -> &'static mut FrameBuffer {
    unsafe {
        #[allow(static_mut_refs)]
        &mut BUFFER
    }
}

pub fn raw_buffer() -> &'static [u8] {
    unsafe {
        std::slice::from_raw_parts(
            &raw const BUFFER as *const u8,
            std::mem::size_of::<FrameBuffer>(),
        )
    }
}
