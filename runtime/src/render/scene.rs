use std::sync::atomic::{AtomicBool, Ordering};

use super::drawing::{self, Color, Fillable, Point, SCREEN};

pub const LIGHT_COUNT: usize = 100;

const LIGHT_RADIUS: usize = 10;
const PADDING: usize = 100;
const HEIGHT_LIGHT_COUNT: usize = 10;

pub struct Scene {
    full: AtomicBool,
    lights: [Color; LIGHT_COUNT],
}

impl Scene {
    pub fn new() -> Self {
        Self {
            full: AtomicBool::new(true),
            lights: [Color::BLACK; LIGHT_COUNT],
        }
    }

    pub fn reset(&mut self) {
        self.full.store(true, Ordering::Relaxed);

        for light in self.lights.iter_mut() {
            *light = Color::BLACK;
        }
    }

    pub fn set_light_color(&mut self, index: usize, color: Color) {
        self.lights[index] = color;
    }

    pub fn render(&self) {
        if self.full.load(Ordering::Relaxed) {
            self.render_background();
            self.full.store(false, Ordering::Relaxed);
        }

        self.render_lights();
    }

    fn render_background(&self) {
        drawing::clear(Color::BLACK);
    }

    fn render_lights(&self) {
        const PADDING_TO_CENTER: usize = PADDING + LIGHT_RADIUS;
        const WIDTH_LIGHT_COUNT: usize = LIGHT_COUNT / HEIGHT_LIGHT_COUNT;
        const SPACE_BETWEEN_LIGHTS_W: usize = (SCREEN.size().width() - PADDING_TO_CENTER * 2) / (WIDTH_LIGHT_COUNT - 1);
        const SPACE_BETWEEN_LIGHTS_H: usize = (SCREEN.size().height() - PADDING_TO_CENTER * 2) / (HEIGHT_LIGHT_COUNT - 1);
        
        for x in 0..WIDTH_LIGHT_COUNT {
            for y in 0..HEIGHT_LIGHT_COUNT {
                let center = Point::new(
                    (PADDING_TO_CENTER + x * SPACE_BETWEEN_LIGHTS_W) as isize,
                    (PADDING_TO_CENTER + y * SPACE_BETWEEN_LIGHTS_H) as isize,
                );

                // bottom to top
                // odd lines : left to right
                // even lines : right to left
                let y_index = HEIGHT_LIGHT_COUNT - y - 1;
                let index = if y_index % 2 == 0 {
                    y_index * WIDTH_LIGHT_COUNT + x
                } else {
                    y_index * WIDTH_LIGHT_COUNT + WIDTH_LIGHT_COUNT - x - 1
                };

                drawing::Circle::new(center, LIGHT_RADIUS).fill(self.lights[index]);
            }
        }
    }
}
