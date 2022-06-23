use monoid_derive::*;

mod last;
mod sum;

pub use last::Last;
pub use sum::Sum;

use crate::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}

#[derive(Debug, Semigroup)]
pub struct Product<T>(pub T);

impl<T: Semigroup + num_traits::Num> Monoid for Product<T> {
    fn empty() -> Self {
        Self(num_traits::identities::One::one())
    }
}

impl<T> From<T> for Product<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Semigroup + Default> Monoid for T {
    fn empty() -> Self {
        Default::default()
    }
}

#[macro_export]
#[deprecated(note="This will be removed, I am unsure why this was here in the first place")]
macro_rules! monoid_default {
    ($t:ty : $($i:ident),*) => {
        impl Monoid for $t {
            fn empty() -> Self {
                Self {
                    $(
                        $i: Monoid::empty(),
                    )*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::last::Last;
    use super::sum::Sum;

    #[test]
    fn combine_with_into() {
        let x = Last::empty()
            .combine(53.into())
            .combine(None.into())
            .combine(42.into());

        assert_eq!(x.0, Some(42));
    }

    #[test]
    fn sum_test() {
        let nums = vec![10, 24, 3, 7, 42];
        let sum = nums
            .into_iter()
            .fold(Sum::empty(), |acc, x| acc.combine(Sum::from(x)));

        assert_eq!(sum, 86);
    }

    #[test]
    fn option_sum() {
        let sum = None
            .combine(Some(Sum::from(10)))
            .combine(None)
            .combine(Some(Sum::from(5)))
            .combine(Some(Sum::from(7)))
            .combine(None)
            .combine(Some(Sum::from(42)))
            .combine(None);

        assert_eq!(sum.unwrap(), 64);
    }

    #[test]
    fn option_combine_macro() {
        let sum: Option<Sum<i32>> = crate::combine!(
            None =>
            Sum::from(10),
            None,
            Sum::from(5),
            Sum::from(7),
            None,
            Sum::from(42),
            None,
        );

        assert_eq!(sum.unwrap(), 64);
    }

    #[test]
    fn combine_macro() {
        let x = crate::combine! {
            Last::from(53) => None, 42, {let b = None; b},
        };

        assert_eq!(x.0, Some(42));
    }

    #[test]
    fn last_to_option_conversion() {
        let last = Last::from(42);
        let res: Option<i32> = last.into();

        assert_eq!(res, Some(42));
    }
}
