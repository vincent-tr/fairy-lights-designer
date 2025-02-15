mod shapes;
pub use shapes::*;

pub use super::frame::Color;

use super::frame::{ WIDTH, HEIGHT };

pub const SCREEN: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(WIDTH, HEIGHT));

pub fn clear(color: Color) {
    draw_rect(&SCREEN, color);
}

pub fn draw_rect(rect: &Rectangle, color: Color) {
    let clamped = rect.clamp(&SCREEN);

    // log::debug!("Drawing rect {:?} with color {:?}", rect, color);

    for x in clamped.left()..clamped.right() {
        for y in clamped.top()..clamped.bottom() {
            let pixel = super::frame::frame().pixel_mut(x as usize, y as usize);
            *pixel = color;
        }
    }
}
