use std::ops::AddAssign;

// use serde::{Serialize, Deserialize};

use super::Component;
// use super::ComponentFields;

type Point = (i32, i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub value: Option<String>,
    prototype: Option<String>,
}
// impl Component for Name {}
impl Component for Name {

    type ValueType = String;

    fn new(value: String) -> Self {
        Self {
            value: None,
            prototype: Some(value),
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        self.value = Some(value);
    }
}

impl Clone for Name {
    fn clone(&self) -> Self {
        Self {
            value: self.prototype.clone(),
            prototype: None
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct  Kind {
    pub value: Option<String>,
    prototype: Option<String>,
}

// impl Component for Kind {}
impl Component for Kind {

    type ValueType = String;

    fn new(value: Self::ValueType) -> Self {
        Self {
            value: None,
            prototype: Some(value)
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        self.value = Some(value);
    }

}

impl Clone for Kind {
    fn clone(&self) -> Self {
        Self {
            value: self.prototype.clone(),
            prototype: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  Location {
    pub value: Option<String>,
    prototype: Option<String>,
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Self {
            value: self.prototype.clone(),
            prototype: None
        }
    }
}
// impl Component for Location {}
impl Component  for  Location {

    type ValueType = String;

    fn new(value: Self::ValueType) -> Self {
        Self {
            value: None,
            prototype: Some(value)
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        self.value = Some(value);
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct  Position {
    pub x: Option<i32>,
    pub y: Option<i32>,
    prototype: Option<Point>,
}


// impl Component for Position {}
impl Component for Position {

    type ValueType = Option<Point>;

    fn new(value: Self::ValueType) -> Self {
        match value {
            Some(v) => Self { x: None, y: None, prototype: value },
            None => Self { ..Default::default() }
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        match value {
            Some(v) =>  {
                self.x = Some(v.0);
                self.y = Some(v.1);
            }
            None => ()
        }
    }

}

impl Default for Position {

    fn default() -> Self {
        Self { x: None, y: None, prototype: Some((0, 0)) }
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Self {
            x: Some(self.prototype.unwrap().0.clone()),
            y: Some(self.prototype.unwrap().1.clone()),
            prototype: None,
        }
    }
}
impl AddAssign for Position {

    fn add_assign(&mut self, other: Position) {
        *self = Position {
            x: Some(self.x.unwrap() + other.x.unwrap()),
            y: Some(self.y.unwrap() + other.y.unwrap()),
            prototype: None
        }
    }
}

impl Position {

    fn move_to(&mut self, value: Point) {
        self.x = Some(value.0);
        self.y = Some(value.1);
    }

    fn to_point(&self) -> Point {
        (self.x.unwrap(), self.y.unwrap())
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Char {
    pub value: Option<char>,
    prototype: Option<char>
}

// impl Component for Char {}
impl Component for Char {
    type ValueType = char;

    fn new(value: Self::ValueType) -> Self {
        Self {
            value: None,
            prototype: Some(value)
        }
    }

    fn update(&mut self, value: Self::ValueType) {
        self.value = Some(value);
    }

}
