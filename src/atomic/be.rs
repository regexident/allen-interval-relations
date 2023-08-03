use core::cmp::Ordering;

use crate::{Bound, IntervalError};

/// Ordering of the start bound of interval `s` and the end bound of interval `t`.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Be(pub(crate) Ordering);

impl Be {
    #[inline]
    pub(crate) fn from_bounds<T>(s: &Bound<T>, t: &Bound<T>) -> Self
    where
        T: Ord,
    {
        Self(match (s, t) {
            (Bound::Bounded(s), Bound::Bounded(t)) => s.cmp(t),
            (Bound::Bounded(_), Bound::Unbounded) => Ordering::Less,
            (Bound::Unbounded, Bound::Bounded(_)) => Ordering::Less,
            (Bound::Unbounded, Bound::Unbounded) => Ordering::Less,
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
            (Bound::Unbounded, Bound::Bounded(_)) => Ok(Ordering::Less),
            (Bound::Unbounded, Bound::Unbounded) => Ok(Ordering::Less),
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
            Be::from_bounds(&Bound::Bounded(isize::MIN), &Bound::Bounded(0)),
            Be(Ordering::Less)
        );
        assert_eq!(
            Be::from_bounds(&Bound::Bounded(0), &Bound::Bounded(0)),
            Be(Ordering::Equal)
        );
        assert_eq!(
            Be::from_bounds(&Bound::Bounded(isize::MAX), &Bound::Bounded(0)),
            Be(Ordering::Greater)
        );
    }

    #[test]
    fn bounded_unbounded() {
        assert_eq!(
            Be::from_bounds(&Bound::Bounded(isize::MIN), &Bound::Unbounded),
            Be(Ordering::Less)
        );
        assert_eq!(
            Be::from_bounds(&Bound::Bounded(0), &Bound::Unbounded),
            Be(Ordering::Less)
        );
        assert_eq!(
            Be::from_bounds(&Bound::Bounded(isize::MAX), &Bound::Unbounded),
            Be(Ordering::Less)
        );
    }

    #[test]
    fn unbounded_bounded() {
        assert_eq!(
            Be::from_bounds(&Bound::Unbounded, &Bound::Bounded(isize::MIN)),
            Be(Ordering::Less)
        );
        assert_eq!(
            Be::from_bounds(&Bound::Unbounded, &Bound::Bounded(0)),
            Be(Ordering::Less)
        );
        assert_eq!(
            Be::from_bounds(&Bound::Unbounded, &Bound::Bounded(isize::MAX)),
            Be(Ordering::Less)
        );
    }

    #[test]
    fn unbounded_unbounded() {
        assert_eq!(
            Be::from_bounds::<isize>(&Bound::Unbounded, &Bound::Unbounded),
            Be(Ordering::Less)
        );
    }
}
