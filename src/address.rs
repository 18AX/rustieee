#[const_trait]
pub trait Address {
    type Inner;

    fn broadcast() -> Self;
    fn value(&self) -> Self::Inner;
}
