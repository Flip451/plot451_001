use anyhow::Result;

pub trait Specification {
    type T;
    type Error;

    fn is_satisfied_by(&self, obj: &Self::T) -> Result<(), Self::Error>;
}