use crate::semigroup::Semigroup;

/// Semigroup of the Sum of type T if type T implements [std::ops::Add].
/// ```
/// # use partial_functional::prelude::*;
///
/// let five = Sum(5);
/// let ten = Sum(10);
///
/// assert_eq!(Sum(15), five.combine(ten));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum<T>(pub T);

impl<T: Default + Semigroup> Default for Sum<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

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

macro_rules! impl_from {
    (
        $($t:ty),* $(,)?
    ) => {
        $(
            impl From<Sum<$t>> for $t {
                fn from(value: Sum<$t>) -> Self {
                    value.0
                }
            }
        )*
    }
}

impl_from!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

impl<T: std::ops::Add<Output = T>> Semigroup for Sum<T> {
    fn combine(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::Monoid;

    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Sum<u32> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Sum(u8::arbitrary(g).into())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(u32::shrink(&self.0).map(|x| x.into()))
        }
    }

    #[quickcheck]
    fn identity_property() {
        assert_eq!(Sum::<u32>(0), Sum::empty())
    }

    #[quickcheck]
    fn sum_of_vec_is_same_as_sum_combine(vec: Vec<u8>) -> bool {
        let left: Sum<u32> = vec.iter().copied().map(|x| x as u32).sum::<u32>().into();
        let right = vec
            .iter()
            .copied()
            .map(|x| Sum::from(x as u32))
            .fold(Sum::default(), |a, x| a.combine(x));

        left == right
    }

    #[quickcheck]
    fn associativity_property(x: Sum<u32>, y: Sum<u32>, z: Sum<u32>) -> bool {
        x.combine(y.combine(z)) == x.combine(y).combine(z)
    }
}
