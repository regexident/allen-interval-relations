use core::cmp::Ordering;

use crate::{Bound, IntervalError};

/// Ordering of the end bound of interval `s` and the end bound of interval `t`.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Ee(pub(crate) Ordering);

impl Ee {
    #[inline]
    pub(crate) fn from_bounds<T>(s: &Bound<T>, t: &Bound<T>) -> Self
    where
        T: Ord,
    {
        Self(match (s, t) {
            (Bound::Bounded(s), Bound::Bounded(t)) => s.cmp(t),
            (Bound::Bounded(_), Bound::Unbounded) => Ordering::Less,
            (Bound::Unbounded, Bound::Bounded(_)) => Ordering::Greater,
            (Bound::Unbounded, Bound::Unbounded) => Ordering::Equal,
        })
    }

    #[inline]
    pub(crate) fn try_from_bounds<T>(s: &Bound<T>, t: &Bound<T>) -> Result<Self, IntervalError>
    where
        T: PartialOrd<T>,
    {
        match (s, t) {
            (Bound::Bounded(s), Bound::Bounded(t)) => {
                s.partial_cmp(t).ok_or(IntervalError::AmbiguousOrder)
            }
            (Bound::Bounded(_), Bound::Unbounded) => Ok(Ordering::Less),
            (Bound::Unbounded, Bound::Bounded(_)) => Ok(Ordering::Greater),
            (Bound::Unbounded, Bound::Unbounded) => Ok(Ordering::Equal),
        }
        .map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_bounded() {
        assert_eq!(
            Ee::from_bounds(&Bound::Bounded(isize::MIN), &Bound::Bounded(0)),
            Ee(Ordering::Less)
        );
        assert_eq!(
            Ee::from_bounds(&Bound::Bounded(0), &Bound::Bounded(0)),
            Ee(Ordering::Equal)
        );
        assert_eq!(
            Ee::from_bounds(&Bound::Bounded(isize::MAX), &Bound::Bounded(0)),
            Ee(Ordering::Greater)
        );
    }

    #[test]
    fn bounded_unbounded() {
        assert_eq!(
            Ee::from_bounds(&Bound::Bounded(isize::MIN), &Bound::Unbounded),
            Ee(Ordering::Less)
        );
        assert_eq!(
            Ee::from_bounds(&Bound::Bounded(0), &Bound::Unbounded),
            Ee(Ordering::Less)
        );
        assert_eq!(
            Ee::from_bounds(&Bound::Bounded(isize::MAX), &Bound::Unbounded),
            Ee(Ordering::Less)
        );
    }

    #[test]
    fn unbounded_bounded() {
        assert_eq!(
            Ee::from_bounds(&Bound::Unbounded, &Bound::Bounded(isize::MIN)),
            Ee(Ordering::Greater)
        );
        assert_eq!(
            Ee::from_bounds(&Bound::Unbounded, &Bound::Bounded(0)),
            Ee(Ordering::Greater)
        );
        assert_eq!(
            Ee::from_bounds(&Bound::Unbounded, &Bound::Bounded(isize::MAX)),
            Ee(Ordering::Greater)
        );
    }

    #[test]
    fn unbounded_unbounded() {
        assert_eq!(
            Ee::from_bounds::<isize>(&Bound::Unbounded, &Bound::Unbounded),
            Ee(Ordering::Equal)
        );
    }
}
