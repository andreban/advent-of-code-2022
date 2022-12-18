#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Point) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
