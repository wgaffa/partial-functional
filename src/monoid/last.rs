use crate::semigroup::Semigroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[cfg(test)]
mod tests {
    use crate::monoid::Monoid;

    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Last<u32> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Last(Option::<u32>::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(Option::<u32>::shrink(&self.0).map(|x| x.into()))
        }
    }

    #[quickcheck]
    fn identity_property() {
        assert_eq!(Last::<u32>(None), Last::empty())
    }

    #[quickcheck]
    fn last_is_set_to_the_last_non_none_value_in_a_list(vec: Vec<Last<u32>>) -> bool {
        let last = vec
            .iter()
            .copied()
            .rfind(|x| *x != Last(None))
            .unwrap_or_default();
        let right = vec
            .iter()
            .copied()
            .fold(Last::default(), |a, x| a.combine(x));

        last == right
    }

    #[quickcheck]
    fn associativity_property(x: Last<u32>, y: Last<u32>, z: Last<u32>) -> bool {
        x.combine(y.combine(z)) == x.combine(y).combine(z)
    }
}
