use crate::semigroup::Semigroup;
use num_traits::Bounded;

/// Semigroup over the Minimum of T if T implements [std::cmp::Ord].
/// ```
/// # use partial_functional::{Min, Semigroup};
/// assert_eq!(Min(5), Min(10).combine(Min(5)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Min<T>(pub T);

impl<T: Default + Semigroup + Bounded> Default for Min<T> {
    fn default() -> Self {
        Self(Bounded::max_value())
    }
}

impl<T: PartialEq> PartialEq<T> for Min<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Min<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T> From<T> for Min<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Ord> Semigroup for Min<T> {
    fn combine(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::Monoid;

    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Min<u32> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Min(u32::arbitrary(g).into())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(u32::shrink(&self.0).map(|x| x.into()))
        }
    }

    #[quickcheck]
    fn identity_property() {
        assert_eq!(Min::<u32>(u32::MAX), Min::empty())
    }

    #[quickcheck]
    fn min_of_vec_is_same_as_min_combine(vec: Vec<Min<u32>>) -> bool {
        let left: Min<u32> = vec
            .iter()
            .copied()
            .map(|x| x.0)
            .min()
            .unwrap_or(u32::MAX)
            .into();
        let right = vec
            .iter()
            .copied()
            .fold(Min::default(), |a, x| a.combine(x));

        left == right
    }

    #[quickcheck]
    fn associativity_property(x: Min<u32>, y: Min<u32>, z: Min<u32>) -> bool {
        x.combine(y.combine(z)) == x.combine(y).combine(z)
    }
}
