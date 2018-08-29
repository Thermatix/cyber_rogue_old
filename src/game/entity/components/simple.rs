use super::Component;

type Point = (i32, i32);

pub struct Name{
    pub value: String,
}

impl<'c> Component <'c> for Name {

    const IDName: &'c str = "name";
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

impl<'c> Component <'c> for  Kind{

    const IDName: &'c str = "kind";
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

impl<'c> Component <'c> for  Location{

    const IDName: &'c str = "location";
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

impl<'c> Component <'c> for  Position{

    const IDName: &'c str = "position";
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
