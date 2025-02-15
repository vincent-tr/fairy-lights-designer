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

impl Add<Size> for Point {
    type Output = Point;

    fn add(self, other: Size) -> Point {
        Point::new(self.x + other.width as isize, self.y + other.height as isize)
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

    pub fn new_with_points(p1: &Point, p2: &Point) -> Self {
        let origin = Point::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let target = Point::new(p1.x.max(p2.x), p1.y.max(p2.y));
        let size = target.checked_sub(origin).unwrap();

        Self {
            origin,
            size,
        }
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

    pub fn left(&self) -> isize {
        self.origin.x
    }

    pub fn right(&self) -> isize {
        self.origin.x + self.size.width as isize
    }

    pub fn top(&self) -> isize {
        self.origin.y
    }

    pub fn bottom(&self) -> isize {
        self.origin.y + self.size.height as isize
    }

    pub const fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x
            && point.x < self.origin.x + self.size.width as isize
            && point.y >= self.origin.y
            && point.y < self.origin.y + self.size.height as isize
    }

    pub fn clamp(&self, boundaries: &Rectangle) -> Rectangle {
        let boundary_top_left = boundaries.top_left();
        let boundary_bottom_right = boundaries.bottom_right();

        let mut top_left = self.top_left();
        let mut bottom_right = self.bottom_right();

        if top_left.x < boundary_top_left.x {
            top_left.x = boundary_top_left.x;
        }
        if top_left.y < boundary_top_left.y {
            top_left.y = boundary_top_left.y;
        }
        if bottom_right.x > boundary_bottom_right.x {
            bottom_right.x = boundary_bottom_right.x;
        }
        if bottom_right.y > boundary_bottom_right.y {
            bottom_right.y = boundary_bottom_right.y;
        }

        return Rectangle::new_with_points(&top_left, &bottom_right);
    }
}
