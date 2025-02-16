mod shapes;
use js_sys::Math::{round, sqrt};
pub use shapes::*;

pub use super::frame::Color;

use super::frame::{ WIDTH, HEIGHT };

pub const SCREEN: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(WIDTH, HEIGHT));

pub fn clear(color: Color) {
    SCREEN.fill(color);
}

pub trait Drawable {
    fn draw(&self, color: Color);
}

pub trait Fillable {
    fn fill(&self, color: Color);
}

impl Drawable for Point {
    fn draw(&self, color: Color) {
        if SCREEN.contains(self) {
            let pixel = super::frame::frame().pixel_mut(self.x() as usize, self.y() as usize);
            *pixel = color;
        }
    }
}

impl Drawable for Line {
    fn draw(&self, color: Color) {
        let dx = self.end().x() - self.start().x();
        let dy = self.end().y() - self.start().y();
    
        let steps = dx.abs().max(dy.abs());
    
        for i in 0..=steps {
            let x = self.start().x() + (dx * i) / steps;
            let y = self.start().y() + (dy * i) / steps;
    
            Point::new(x, y).draw(color);
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, color: Color) {
        Line::new(self.top_left(), self.top_right()).draw(color);
        Line::new(self.top_right(), self.bottom_right()).draw(color);
        Line::new(self.bottom_right(), self.bottom_left()).draw(color);
        Line::new(self.bottom_left(), self.top_left()).draw(color);
    }
}

impl Drawable for Circle {
    fn draw(&self, color: Color) {
        let mut x = self.radius() as isize;
        let mut y = 0;
        let mut err = 0;
    
        while x >= y {
            (self.center() + Offset::new(x, y)).draw(color);
            (self.center() + Offset::new(y, x)).draw(color);
            (self.center() + Offset::new(-y, x)).draw(color);
            (self.center() + Offset::new(-x, y)).draw(color);
    
            (self.center() + Offset::new(-x, -y)).draw(color);
            (self.center() + Offset::new(-y, -x)).draw(color);
            (self.center() + Offset::new(y, -x)).draw(color);
            (self.center() + Offset::new(x, -y)).draw(color);
        
            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            }
    
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }
}

impl Fillable for Rectangle {
    fn fill(&self, color: Color) {
        for x in self.left()..self.right() {
            for y in self.top()..self.bottom() {
                Point::new(x, y).draw(color);
            }
        }
    }
}

impl Fillable for Circle {
    fn fill(&self, color: Color) {
        let r = self.radius() as isize;
        for x in -r..r {
            let height = round(sqrt((r * r - x * x) as f64)) as isize;

            for y in -height..height {
                (self.center() + Offset::new(x, y)).draw(color);
            }
        }
    }
}

