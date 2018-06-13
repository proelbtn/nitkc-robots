use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: u64,
    pub y: u64,
}

impl Vec2 {
    pub fn new(x: u64, y: u64) -> Vec2 {
        Vec2 { x, y }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}