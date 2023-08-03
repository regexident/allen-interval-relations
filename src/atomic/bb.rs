use core::cmp::Ordering;

use crate::{Bound, IntervalError};

/// Ordering of the start bound of interval `s` and the start bound of interval `t`.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Bb(pub(crate) Ordering);

impl Bb {
    #[inline]
    pub(crate) fn from_bounds<T>(s: &Bound<T>, t: &Bound<T>) -> Self
    where
        T: Ord,
    {
        Self(match (s, t) {
            (Bound::Bounded(s), Bound::Bounded(t)) => s.cmp(t),
            (Bound::Bounded(_), Bound::Unbounded) => Ordering::Greater,
            (Bound::Unbounded, Bound::Bounded(_)) => Ordering::Less,
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
            (Bound::Bounded(_), Bound::Unbounded) => Ok(Ordering::Greater),
            (Bound::Unbounded, Bound::Bounded(_)) => Ok(Ordering::Less),
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
            Bb::from_bounds(&Bound::Bounded(isize::MIN), &Bound::Bounded(0)),
            Bb(Ordering::Less)
        );
        assert_eq!(
            Bb::from_bounds(&Bound::Bounded(0), &Bound::Bounded(0)),
            Bb(Ordering::Equal)
        );
        assert_eq!(
            Bb::from_bounds(&Bound::Bounded(isize::MAX), &Bound::Bounded(0)),
            Bb(Ordering::Greater)
        );
    }

    #[test]
    fn bounded_unbounded() {
        assert_eq!(
            Bb::from_bounds(&Bound::Bounded(isize::MIN), &Bound::Unbounded),
            Bb(Ordering::Greater)
        );
        assert_eq!(
            Bb::from_bounds(&Bound::Bounded(0), &Bound::Unbounded),
            Bb(Ordering::Greater)
        );
        assert_eq!(
            Bb::from_bounds(&Bound::Bounded(isize::MAX), &Bound::Unbounded),
            Bb(Ordering::Greater)
        );
    }

    #[test]
    fn unbounded_bounded() {
        assert_eq!(
            Bb::from_bounds(&Bound::Unbounded, &Bound::Bounded(isize::MIN)),
            Bb(Ordering::Less)
        );
        assert_eq!(
            Bb::from_bounds(&Bound::Unbounded, &Bound::Bounded(0)),
            Bb(Ordering::Less)
        );
        assert_eq!(
            Bb::from_bounds(&Bound::Unbounded, &Bound::Bounded(isize::MAX)),
            Bb(Ordering::Less)
        );
    }

    #[test]
    fn unbounded_unbounded() {
        assert_eq!(
            Bb::from_bounds::<isize>(&Bound::Unbounded, &Bound::Unbounded),
            Bb(Ordering::Equal)
        );
    }
}
