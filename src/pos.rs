use std::{fmt::Display, ops::Add};

use serde_json::{Value, json};

#[derive(Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl Add for Pos {
    type Output = Pos;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
impl Pos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    pub fn to_json(&self) -> Value {
        json!({
            "x": self.x,
            "y": self.y,
            "z": self.z,
        })
    }
    pub fn add_x(&self, x: i32) -> Self {
        let mut new = self.clone();
        new.x += x;
        new
    }
}
impl Default for Pos {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}
