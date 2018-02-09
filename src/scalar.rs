#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Scalar<T> {
    val: T,
}

impl<T> Scalar<T> {
    pub fn new(val: T) -> Scalar<T> {
        Scalar { val }
    }
}
