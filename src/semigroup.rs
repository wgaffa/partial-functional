use std::{marker::PhantomData, cmp::Ordering};

/// The trait combines two types into another one.
///
/// The combining of several types must be associative, meaning that they can be evaluated in any order.
/// `first.combine(second.combine(third)) == first.combine(second).combine(third)`
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

/// Returns the combination of both Some variants, if one is None then the other is returned.
/// ```
/// use partial_functional::semigroup::Semigroup;
///
/// let five = Some(5);
/// let ten = Some(10);
///
/// assert_eq!(Some(15), five.combine(ten));
/// assert_eq!(Some(5), five.combine(None));
/// assert_eq!(Some(10), None.combine(ten));
/// ```
impl<T: Semigroup> Semigroup for Option<T> {
    fn combine(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Some(left), Some(right)) => Some(left.combine(right)),
            (left, right) => left.or(right),
        }
    }
}

/// Returns the first Result if it's an Ok variant, otherwise returns the second
///
/// # Examples
/// ```
/// use partial_functional::semigroup::Semigroup;
///
/// let five: Result<u32, &'static str> = Ok(5);
/// let two_kb: Result<u32, &'static str> = Ok(2048);
/// let err: Result<u32, &'static str> = Err("An error occured");
/// let err_again: Result<u32, &'static str> = Err("Another error");
///
/// assert_eq!(Ok(5), five.combine(err.clone()));
/// assert_eq!(Ok(2048), two_kb.combine(five.clone()));
/// assert_eq!(Ok(2048), two_kb.combine(five));
/// assert_eq!(Err("Another error"), err.combine(err_again));
/// ```
impl<T, E> Semigroup for Result<T, E> {
    fn combine(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Err(_), b) => b,
            (a, _) => a,
        }
    }
}

/// This combines two Ordering operations into one. 'first.cmp(second)' for example gives back an Ordering.
/// If we have several types you wish to compare say name and age for example. We can first order by name and then age.
///
/// # Examples
/// ```
/// # use std::cmp::Ordering;
/// use partial_functional::semigroup::Semigroup;
///
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// let first = Person { name: String::from("Chris"), age: 43 };
/// let second = Person { name: String::from("Chris"), age: 23 };
///
/// let fst_compared_to_snd = first.name.cmp(&second.name).combine(first.age.cmp(&second.age));
///
/// assert_eq!(Ordering::Greater, fst_compared_to_snd);
/// ```
impl Semigroup for Ordering {
    fn combine(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Ordering::Less, _) => Ordering::Less,
            (Ordering::Equal, y) => y,
            (Ordering::Greater, _) => Ordering::Greater,
        }
    }
}

impl Semigroup for String {
    fn combine(self, rhs: Self) -> Self {
        self + &rhs
    }
}

impl<T> Semigroup for Vec<T> {
    fn combine(mut self, rhs: Self) -> Self {
        self.extend(rhs);
        self
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
