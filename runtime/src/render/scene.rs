use std::sync::atomic::{AtomicBool, Ordering};

use super::drawing::{clear, Circle, Color, Drawable, Fillable, Line, Point, SCREEN};

pub struct Scene {
    full: AtomicBool,
    lights: [Color; Self::LIGHT_COUNT],
}

impl Scene {
    pub const LIGHT_COUNT: usize = 100;

    const LIGHT_RADIUS: usize = 10;
    const PADDING: usize = 100;
    const HEIGHT_LIGHT_COUNT: usize = 10;

    // computed values
    const PADDING_TO_CENTER: usize = Self::PADDING + Self::LIGHT_RADIUS;
    const WIDTH_LIGHT_COUNT: usize = Self::LIGHT_COUNT / Self::HEIGHT_LIGHT_COUNT;
    const SPACE_BETWEEN_LIGHTS_W: usize =
        (SCREEN.size().width() - Self::PADDING_TO_CENTER * 2) / (Self::WIDTH_LIGHT_COUNT - 1);
    const SPACE_BETWEEN_LIGHTS_H: usize =
        (SCREEN.size().height() - Self::PADDING_TO_CENTER * 2) / (Self::HEIGHT_LIGHT_COUNT - 1);

    pub fn new() -> Self {
        Self {
            full: AtomicBool::new(true),
            lights: [Color::BLACK; Self::LIGHT_COUNT],
        }
    }

    pub fn reset(&mut self) {
        self.full.store(true, Ordering::Relaxed);

        for light in self.lights.iter_mut() {
            *light = Color::BLACK;
        }
    }

    pub fn get_light_color(&self, index: usize) -> Color {
        self.lights[index]
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
        let mut points: [Point; Self::LIGHT_COUNT] = [Point::new(0, 0); Self::LIGHT_COUNT];

        for x in 0..Self::WIDTH_LIGHT_COUNT {
            for y in 0..Self::HEIGHT_LIGHT_COUNT {
                let center = Point::new(
                    (Self::PADDING_TO_CENTER + x * Self::SPACE_BETWEEN_LIGHTS_W) as isize,
                    (Self::PADDING_TO_CENTER + y * Self::SPACE_BETWEEN_LIGHTS_H) as isize,
                );

                let index = self.coord_to_index(x, y);

                points[index] = center;
            }
        }

        clear(Color::BLACK);

        for i in 0..(Self::LIGHT_COUNT - 1) {
            let line = Line::new(points[i], points[i + 1]);
            line.draw(Color::WHITE);
        }

        for x in 0..Self::WIDTH_LIGHT_COUNT {
            for y in 0..Self::HEIGHT_LIGHT_COUNT {
                let center = Point::new(
                    (Self::PADDING_TO_CENTER + x * Self::SPACE_BETWEEN_LIGHTS_W) as isize,
                    (Self::PADDING_TO_CENTER + y * Self::SPACE_BETWEEN_LIGHTS_H) as isize,
                );

                Circle::new(center, Self::LIGHT_RADIUS + 1).fill(Color::WHITE);
                Circle::new(center, Self::LIGHT_RADIUS).fill(Color::BLACK);
            }
        }
    }

    fn render_lights(&self) {
        for x in 0..Self::WIDTH_LIGHT_COUNT {
            for y in 0..Self::HEIGHT_LIGHT_COUNT {
                let center = Point::new(
                    (Self::PADDING_TO_CENTER + x * Self::SPACE_BETWEEN_LIGHTS_W) as isize,
                    (Self::PADDING_TO_CENTER + y * Self::SPACE_BETWEEN_LIGHTS_H) as isize,
                );

                let index = self.coord_to_index(x, y);
                Circle::new(center, Self::LIGHT_RADIUS).fill(self.lights[index]);
            }
        }
    }

    fn coord_to_index(&self, x: usize, y: usize) -> usize {
        // bottom to top
        // odd lines : left to right
        // even lines : right to left
        let y_index = Self::HEIGHT_LIGHT_COUNT - y - 1;
        if y_index % 2 == 0 {
            y_index * Self::WIDTH_LIGHT_COUNT + x
        } else {
            y_index * Self::WIDTH_LIGHT_COUNT + Self::WIDTH_LIGHT_COUNT - x - 1
        }
    }
}
