pub trait Component<'c> {

    const IDName: &'c str;
    type ValueType;

    fn new(value: Self::ValueType) -> Self;

    fn update(&mut self, value: Self::ValueType);
}


