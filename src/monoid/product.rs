use num_traits;

use crate::semigroup::Semigroup;

/// Semigroup of the Product of type T if type T implements num_traits::Num.
/// ```
/// # use partial_functional::prelude::*;
/// let five = Product(5);
/// let ten = Product(10);
/// assert_eq!(Product(50), five.combine(ten));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Product<T>(pub T);

impl<T: Semigroup + num_traits::Num> Default for Product<T> {
    fn default() -> Self {
        Self(num_traits::identities::One::one())
    }
}

impl<T: PartialEq> PartialEq<T> for Product<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Product<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}
impl<T> From<T> for Product<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: std::ops::Mul<Output = T>> Semigroup for Product<T> {
    fn combine(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::Monoid;

    use super::*;

    use quickcheck::{Arbitrary, Gen, TestResult};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Product<u32> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Product(u8::arbitrary(g).into())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(u32::shrink(&self.0).map(|x| x.into()))
        }
    }

    #[quickcheck]
    fn identity_property() {
        assert_eq!(Product::<u32>(1), Product::empty())
    }

    fn property_product_of_vec_is_same_as_product_combine(vec: Vec<u8>) -> TestResult {
        if vec.len() < 2 {
            return TestResult::discard();
        }

        let left: Product<u128> = vec
            .iter()
            .copied()
            .map(|x| x as u128)
            .product::<u128>()
            .into();
        let right = vec
            .iter()
            .copied()
            .map(|x| Product::from(x as u128))
            .fold(Product::default(), |a, x| a.combine(x));

        TestResult::from_bool(left == right)
    }

    #[test]
    fn product_of_vec_is_same_as_product_combine() {
        let gen = Gen::new(15);
        let mut qtest = quickcheck::QuickCheck::new().gen(gen);

        qtest.quickcheck(property_product_of_vec_is_same_as_product_combine as fn(Vec<u8>) -> TestResult);
    }

    #[quickcheck]
    fn associativity_property(x: Product<u32>, y: Product<u32>, z: Product<u32>) -> bool {
        x.combine(y.combine(z)) == x.combine(y).combine(z)
    }
}
