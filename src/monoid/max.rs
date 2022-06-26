use crate::semigroup::Semigroup;
use num_traits::Bounded;

/// Semigroup over the Maximum of T if T implements [std::cmp::Ord].
/// ```
/// # use partial_functional::{Max, Semigroup};
/// assert_eq!(Max(10), Max(10).combine(Max(5)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Max<T>(pub T);

impl<T: Default + Semigroup + Bounded> Default for Max<T> {
    fn default() -> Self {
        Self(Bounded::min_value())
    }
}

impl<T: PartialEq> PartialEq<T> for Max<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Max<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T> From<T> for Max<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Ord> Semigroup for Max<T> {
    fn combine(self, rhs: Self) -> Self {
        Self(self.0.max(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::Monoid;

    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Max<u32> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Max(u32::arbitrary(g).into())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(u32::shrink(&self.0).map(|x| x.into()))
        }
    }

    #[quickcheck]
    fn identity_property() {
        assert_eq!(Max::<u32>(u32::MIN), Max::empty())
    }

    #[quickcheck]
    fn max_of_vec_is_same_as_max_combine(vec: Vec<Max<u32>>) -> bool {
        let left: Max<u32> = vec
            .iter()
            .copied()
            .map(|x| x.0)
            .max()
            .unwrap_or(u32::MIN)
            .into();
        let right = vec
            .iter()
            .copied()
            .fold(Max::default(), |a, x| a.combine(x));

        left == right
    }

    #[quickcheck]
    fn associativity_property(x: Max<u32>, y: Max<u32>, z: Max<u32>) -> bool {
        x.combine(y.combine(z)) == x.combine(y).combine(z)
    }
}
