mod last;
mod first;
mod sum;
mod product;
mod any;
mod all;
mod min;
mod max;

pub use self::{
    last::Last,
    first::First,
    sum::Sum,
    product::Product,
    any::Any,
    all::All,
    min::Min,
    max::Max,
};

use crate::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
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
    fn option_combine_macro() {
        let sum: Option<Sum<i32>> = crate::combine!(
            None,
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
            Last::from(53), None, 42, {let b = None; b},
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
