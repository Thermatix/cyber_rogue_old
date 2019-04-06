use std::ops::AddAssign;

// use serde::{Serialize, Deserialize};

use super::Component;
// use super::ComponentFields;

type Point = (i32, i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub value: String,
}
// impl Component for Name {}
impl Component for Name {

    type ValueType = String;

    fn new(value: String) -> Self {
        Self {
            value
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        self.value = value;
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct  Kind {
    pub value: String,
}

// impl Component for Kind {}
impl Component for Kind {

    type ValueType = String;

    fn new(value: Self::ValueType) -> Self {
        Self {
            value
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        self.value = value;
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct  Location {
    pub value: String,
}

// impl Component for Location {}
impl Component  for  Location {

    type ValueType = String;

    fn new(value: Self::ValueType) -> Self {
        Self {
            value
        }
    }


    fn update(&mut self, value: Self::ValueType) {
        self.value = value;
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct  Position {
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
}


// impl Component for Position {}
impl Component for Position {

    type ValueType = Option<Point>;

    fn new(value: Self::ValueType) -> Self {
        match value {
            Some(v) => Self { x: v.0, y: v.1 },
            None => Self { ..Default::default() }
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        match value {
            Some(v) =>  {
                self.x = v.0;
                self.y = v.1;
            }
            None => ()
        }
    }

}

impl Default for Position {

    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl AddAssign for Position {

    fn add_assign(&mut self, other: Position) {
        *self = Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Position {

    fn move_to(&mut self, value: Point) {
        self.x = value.0;
        self.y = value.1;
    }

    fn to_point(&self) -> Point {
        (self.x, self.y)
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Char { value: char }

// impl Component for Char {}
impl Component for Char {
    type ValueType = char;

    fn new(value: Self::ValueType) -> Self {
        Self { value }
    }

    fn update(&mut self, value: Self::ValueType) {
        self.value = value;
    }

}
