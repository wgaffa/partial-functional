use std::marker::PhantomData;

pub trait Semigroup {
    fn combine(self, rhs: Self) -> Self;
}

#[macro_export]
macro_rules! combine {
    ( $init:expr => $($x:expr),+ $(,)? ) => {
        $init$(
            .combine($x.into())
        )*
    };
}

impl<T: Semigroup> Semigroup for Option<T> {
    fn combine(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Some(left), Some(right)) => Some(left.combine(right)),
            (left, right) => left.or(right),
        }
    }
}

macro_rules! impl_semigroup_with_addition {
    ( $($x:ty),* ) => {
        $(
            impl Semigroup for $x {
                fn combine(self, rhs: Self) -> Self {
                    self + rhs
                }
            }
        )*
    };
}

impl_semigroup_with_addition!(
    usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64
);

impl Semigroup for bool {
    fn combine(self, rhs: Self) -> Self {
        self || rhs
    }
}

impl<T> Semigroup for PhantomData<T> {
    fn combine(self, _rhs: Self) -> Self {
        self
    }
}

#[macro_export]
#[deprecated(note="This will be replaced, or already is replaced with a derive macro")]
macro_rules! semigroup_default {
    ($t:ty : $($i:ident),*) => {
        impl Semigroup for $t {
            fn combine(self, rhs: Self) -> Self {
                Self {
                    $(
                        $i: self.$i.combine(rhs.$i),
                    )*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monoid::{Last, Sum};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn combine_option_is_the_sum_of_both(left: Option<u32>, right: Option<u32>) -> bool {
        // cast everything to u64 so that we don't overflow when adding them up
        let sum = match (left, right) {
            (None, None) => None,
            (None, Some(x)) => Some(x as u64),
            (Some(x), None) => Some(x as u64),
            (Some(x), Some(y)) => Some(x as u64 + y as u64),
        };

        let left = left.map(|x| x as u64);
        let right = right.map(|x| x as u64);
        let result = left.combine(right);

        sum == result
    }

    #[quickcheck]
    fn option_associativity_property(x: Option<u8>, y: Option<u8>, z: Option<u8>) -> bool {
        let (x, y, z) = (x.map(|x| x as u16), y.map(|x| x as u16), z.map(|x| x as u16));

        x.combine(y.combine(z)) == x.combine(y).combine(z)
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
}
