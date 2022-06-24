use crate::semigroup::Semigroup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct All(pub bool);

impl Semigroup for All {
    fn combine(self, rhs: Self) -> Self {
        Self(self.0 && rhs.0)
    }
}

impl Default for All {
    fn default() -> Self {
        Self(true)
    }
}

impl PartialEq<bool> for All {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<bool> for All {
    fn partial_cmp(&self, other: &bool) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<bool> for All {
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

    impl Arbitrary for All {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            All(<bool>::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(<bool>::shrink(&self.0).map(|x| x.into()))
        }
    }

    #[test]
    fn equality_with_primitive() {
        let any = All::from(true);

        assert_eq!(any, true);
    }

    #[test]
    fn ordering_with_primitive() {
        let any = All::from(true);

        assert_eq!(any > true, false);
    }

    #[quickcheck]
    fn identity_property() {
        assert_eq!(All(true), All::empty())
    }

    #[quickcheck]
    fn any_is_true_if_atleast_one_is_true(vec: Vec<All>) -> bool {
        let left: All = vec
            .iter()
            .copied()
            .all(|x| x == All(true))
            .into();
        let right = vec
            .iter()
            .copied()
            .fold(All::default(), |a, x| a.combine(x));

        left == right
    }

    #[quickcheck]
    fn associativity_property(x: All, y: All, z: All) -> bool {
        x.combine(y.combine(z)) == x.combine(y).combine(z)
    }
}
