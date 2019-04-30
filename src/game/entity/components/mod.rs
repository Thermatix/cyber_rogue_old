mod simple;

pub use self::simple::*;

// pub trait Component {}

// pub trait ComponentFields {
pub trait Component {

    type ValueType;

    fn new(value: Self::ValueType) -> Self;
    // TODO: change all components to set prototype field
    // TODO: change all components to impliment clone that
    // copies the data from prototype into the
    // main fields and then wipes the prototype field

    fn update(&mut self, value: Self::ValueType);

    // fn init(&mut self) -> Self {
    //     self.value = self.prototype
    // }
}
