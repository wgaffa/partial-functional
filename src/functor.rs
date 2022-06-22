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

    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    // fmap id = id
    #[quickcheck]
    fn test_functor_identity_law_with_option(value: Option<u32>) -> bool {
        value.fmap(identity) == identity(value)
    }

    #[quickcheck]
    fn test_functor_identity_law_with_result(value: Result<u32, u8>) -> bool {
        value.fmap(identity) == identity(value)
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

        let add = |x| x + 2; // f
        let multiply = |x| x * 2; // g

        TestResult::from_bool(value.fmap(|x| multiply(add(x))) == value.fmap(add).fmap(multiply))
    }

    #[quickcheck]
    fn test_functor_composition_law_with_result(value: Result<u32, u8>) -> TestResult {
        // This prevents any overflowing numbers to be tested
        if let Ok(val) = value {
            if val >= u32::MAX / 2 - 4 {
                return TestResult::discard();
            }
        }

        let add = |x| x + 2; // f
        let multiply = |x| x * 2; // g

        TestResult::from_bool(value.fmap(|x| multiply(add(x))) == value.fmap(add).fmap(multiply))
    }
}
