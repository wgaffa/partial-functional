use crate::HKT;

pub trait Functor<A, B>: HKT<A, B> {
    fn fmap<F: FnOnce(A) -> B>(self, f: F) -> <Self as HKT<A, B>>::Target;
}

impl<A, B> Functor<A, B> for Option<A> {
    fn fmap<F: FnOnce(A) -> B>(self, f: F) -> <Self as HKT<A, B>>::Target {
        self.map(f)
    }
}

impl<A, B, E> Functor<A, B> for Result<A, E> {
    fn fmap<F: FnOnce(A) -> B>(self, f: F) -> <Self as HKT<A, B>>::Target {
        self.map(f)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::identity;

    use paste::paste;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    // fmap id = id
    macro_rules! functor_identity {
        ( $(($a:ident, $b:ty)),* $(,)? ) => {
            $(
                paste!{
                    #[quickcheck]
                    fn [<functor_identity_law_with_ $a>](value: $b) -> bool {
                        value.fmap(identity) == identity(value)
                    }
                }
            )*
        };
    }

    functor_identity!((option, Option<u32>), (result, Result<u32, u8>),);

    macro_rules! assert_composition {
        ( $name:ident, $f:expr, $g:expr ) => {
            let f = $f;
            let g = $g;

            TestResult::from_bool($name.fmap(|x| g(f(x))) == $name.fmap(f).fmap(g))
        }
    }

    // fmap ( f . g ) == fmap f . fmap g
    #[quickcheck]
    fn test_functor_composition_law_with_option(value: Option<u32>) -> TestResult {
        // This prevents any overflowing numbers to be tested
        if let Some(val) = value {
            if val >= u32::MAX / 2 - 4 {
                return TestResult::discard();
            }
        }

        assert_composition!{
            value,
            |x| x + 2,
            |x| x * 2
        }
    }

    #[quickcheck]
    fn test_functor_composition_law_with_result(value: Result<u32, u8>) -> TestResult {
        // This prevents any overflowing numbers to be tested
        if let Ok(val) = value {
            if val >= u32::MAX / 2 - 4 {
                return TestResult::discard();
            }
        }

        assert_composition!{
            value,
            |x| x + 2,
            |x| x * 2
        }
    }
}
