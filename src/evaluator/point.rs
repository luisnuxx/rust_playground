#![allow(dead_code)]


pub struct Point {
    pub x: i64,
    pub y: i64
}


impl Point {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}", self.x.to_string(), self.y.to_string())
    }
}