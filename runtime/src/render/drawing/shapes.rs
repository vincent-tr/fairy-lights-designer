use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const fn x(&self) -> isize {
        self.x
    }

    pub const fn y(&self) -> isize {
        self.y
    }

    pub fn checked_sub(self, other: Point) -> Option<Size> {
        if self.x < other.x || self.y < other.y {
            None
        } else {
            Some(Size::new((self.x - other.x) as usize, (self.y - other.y) as usize))
        }
    }
}

impl Add<Offset> for Point {
    type Output = Point;

    fn add(self, other: Offset) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    x: isize,
    y: isize,
}

impl Offset {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const fn horizontal(x: isize) -> Self {
        Self { x, y: 0 }
    }

    pub const fn vertical(y: isize) -> Self {
        Self { x: 0, y }
    }
    
    pub const fn x(&self) -> isize {
        self.x
    }
    
    pub const fn y(&self) -> isize {
        self.y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub const fn start(&self) -> Point {
        self.start
    }

    pub const fn end(&self) -> Point {
        self.end
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    width: usize,
    height: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    origin: Point,
    size: Size,
}

impl Rectangle {
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub const fn top_left(&self) -> Point {
        self.origin
    }

    pub const fn size(&self) -> Size {
        self.size
    }

    pub const fn top_right(&self) -> Point {
        Point::new(self.origin.x + self.size.width as isize, self.origin.y)
    }

    pub const fn bottom_left(&self) -> Point {
        Point::new(self.origin.x, self.origin.y + self.size.height as isize)
    }

    pub const fn bottom_right(&self) -> Point {
        Point::new(self.origin.x + self.size.width as isize, self.origin.y + self.size.height as isize)
    }

    pub const fn left(&self) -> isize {
        self.origin.x
    }

    pub const fn right(&self) -> isize {
        self.origin.x + self.size.width as isize
    }

    pub const fn top(&self) -> isize {
        self.origin.y
    }

    pub const fn bottom(&self) -> isize {
        self.origin.y + self.size.height as isize
    }

    pub const fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x
            && point.x < self.origin.x + self.size.width as isize
            && point.y >= self.origin.y
            && point.y < self.origin.y + self.size.height as isize
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    center: Point,
    radius: usize,
}

impl Circle {
    pub const fn new(center: Point, radius: usize) -> Self {
        Self { center, radius }
    }

    pub const fn center(&self) -> Point {
        self.center
    }

    pub const fn radius(&self) -> usize {
        self.radius
    }

    pub const fn contains(&self, point: &Point) -> bool {
        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;
        let distance_sqr = (dx * dx + dy * dy) as usize;
        let radius_sqr = self.radius * self.radius;
        distance_sqr < radius_sqr
    }
}
