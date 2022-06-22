pub mod functor;

pub trait HKT<A, B> {
    type URI;
    type Target;
}

impl<A, B> HKT<A, B> for Option<A> {
    type URI = Self;
    type Target = Option<B>;
}

impl<A, B, E> HKT<A, B> for Result<A, E> {
    type URI = Self;
    type Target = Result<B, E>;
}
