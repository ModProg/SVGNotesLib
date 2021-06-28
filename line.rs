
#[derive(PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub w: f32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(({}, {}), {})", self.x, self.y, self.w)
    }
}

#[derive(Debug)]
pub struct Line {
    pub points: Vec<Point>,
    pub color: Color,
    pub width: f32,
}
