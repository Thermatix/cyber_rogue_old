mod simple;

use sys::entity_manager::ComponentId;

pub use self::simple::*;

pub trait Component {}

pub trait ComponentFields {

    type ValueType;

    fn new(value: Self::ValueType) -> Self;

    fn update(&mut self, value: Self::ValueType);

}
