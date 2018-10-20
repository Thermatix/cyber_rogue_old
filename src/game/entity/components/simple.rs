use super::Component;
// use super::ComponentFields;

use std::ops::AddAssign;

type Point = (i32, i32);

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


pub struct  Position {
    pub x: i32,
    pub y: i32,
}

// impl Component for Position {}
impl Component for Position {

    type ValueType = Point;

    fn new(value: Self::ValueType) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        self.x = value.0;
        self.y = value.1;
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

struct Char { value: char }

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
