use crate::semigroup::Semigroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Any(pub bool);

impl Semigroup for Any {
    fn combine(self, rhs: Self) -> Self {
        Self(self.0 || rhs.0)
    }
}

impl PartialEq<bool> for Any {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<bool> for Any {
    fn partial_cmp(&self, other: &bool) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Default for Any {
    fn default() -> Self {
        Self(false)
    }
}

impl From<bool> for Any {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::Monoid;

    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Any {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Any(<bool>::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(<bool>::shrink(&self.0).map(|x| x.into()))
        }
    }

    #[test]
    fn equality_with_primitive() {
        let any = Any::from(true);

        assert_eq!(any, true);
    }

    #[test]
    fn ordering_with_primitive() {
        let any = Any::from(true);

        assert_eq!(any > true, false);
    }

    #[quickcheck]
    fn identity_property() {
        assert_eq!(Any(false), Any::empty())
    }

    #[quickcheck]
    fn any_is_true_if_atleast_one_is_true(vec: Vec<Any>) -> bool {
        let left: Any = vec
            .iter()
            .copied()
            .any(|x| x == Any(true))
            .into();
        let right = vec
            .iter()
            .copied()
            .fold(Any::default(), |a, x| a.combine(x));

        left == right
    }

    #[quickcheck]
    fn associativity_property(x: Any, y: Any, z: Any) -> bool {
        x.combine(y.combine(z)) == x.combine(y).combine(z)
    }
}
