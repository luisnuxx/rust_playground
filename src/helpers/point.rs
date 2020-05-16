#![allow(dead_code)]

pub struct Point {
    pub x: i64,
    pub y: i64
}

impl Point {
    pub fn new( p_x:i64, p_y: i64) -> Self {
        Self { x: p_x, y: p_y }
    }

    pub fn to_string(&self) -> String {
        format!("[ X: {}, Y: {} ]", self.x.to_string(), self.y.to_string())
    }
}