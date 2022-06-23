use crate::{semigroup::Semigroup, monoid::Monoid};
use monoid_derive::Semigroup;

#[derive(Debug, Semigroup, Monoid, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum<T>(pub T);

impl<T: PartialEq> PartialEq<T> for Sum<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Sum<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T> From<T> for Sum<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
