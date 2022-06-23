use crate::semigroup::Semigroup;

#[derive(Debug, Clone, Copy)]
pub struct Last<T>(pub Option<T>);

impl<T> Default for Last<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> From<T> for Last<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T> From<Option<T>> for Last<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T> From<Last<T>> for Option<T> {
    fn from(value: Last<T>) -> Self {
        value.0
    }
}

impl<T> Semigroup for Last<T> {
    fn combine(self, rhs: Self) -> Self {
        Self(rhs.0.or(self.0))
    }
}
