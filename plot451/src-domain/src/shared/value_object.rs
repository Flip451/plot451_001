pub trait ValueObject: PartialEq
where
    Self: Sized,
{
    type Value;
    type Error;

    fn new(value: Self::Value) -> Result<Self, Self::Error>;
    fn value(&self) -> &Self::Value;
    fn clone_value(&self) -> Self::Value;
}
