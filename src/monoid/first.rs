use crate::semigroup::Semigroup;

/// A semigroup over Option by returning the first value available.
/// ```
/// # use partial_functional::prelude::*;
/// let five = First::from(5);
/// let no_value = First(None);
/// let ten = First::from(10);
/// assert_eq!(First::from(10), ten.combine(five));
/// assert_eq!(First::from(10), ten.combine(no_value));
/// assert_eq!(First(None), no_value.combine(no_value));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct First<T>(pub Option<T>);

impl<T> Default for First<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> From<T> for First<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T> From<Option<T>> for First<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T> From<First<T>> for Option<T> {
    fn from(value: First<T>) -> Self {
        value.0
    }
}

impl<T> Semigroup for First<T> {
    fn combine(self, rhs: Self) -> Self {
        Self(self.0.or(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::Monoid;

    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for First<u32> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            First(Option::<u32>::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(Option::<u32>::shrink(&self.0).map(|x| x.into()))
        }
    }

    #[quickcheck]
    fn identity_property() {
        assert_eq!(First::<u32>(None), First::empty())
    }

    #[quickcheck]
    fn first_is_set_to_the_first_non_none_value_in_a_list(vec: Vec<First<u32>>) -> bool {
        let last = vec
            .iter()
            .copied()
            .find(|x| *x != First(None))
            .unwrap_or_default();
        let right = vec
            .iter()
            .copied()
            .fold(First::default(), |a, x| a.combine(x));

        last == right
    }

    #[quickcheck]
    fn associativity_property(x: First<u32>, y: First<u32>, z: First<u32>) -> bool {
        x.combine(y.combine(z)) == x.combine(y).combine(z)
    }
}
