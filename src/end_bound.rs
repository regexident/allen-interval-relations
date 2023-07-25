use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{Bound, Bounded, Discreteness, StartBound};

/// An upper endpoint of a range of keys.
#[derive(Clone, Copy, Debug)]
pub struct EndBound<T, D> {
    bound: Bound<T>,
    _phantom: PhantomData<D>,
}

impl<T, D> EndBound<T, D>
where
    D: Discreteness,
{
    /// Creates a finite endpoint bounded at `value`.
    pub fn bounded(value: T) -> Self {
        Self::from(Bound::Bounded(value))
    }

    /// Creates an infinite unbounded endpoint.
    pub fn unbounded() -> Self {
        Self::from(Bound::Unbounded)
    }

    /// Returns the underlying bound.
    pub fn bound(&self) -> &Bound<T> {
        &self.bound
    }

    /// Consumes this endpoint, returning the underlying bound.
    pub fn into_bound(self) -> Bound<T> {
        self.bound
    }
}

impl<T, D> From<Bound<T>> for EndBound<T, D>
where
    D: Discreteness,
{
    fn from(value: Bound<T>) -> Self {
        Self {
            bound: value,
            _phantom: PhantomData,
        }
    }
}

impl<T, D> Hash for EndBound<T, D>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bound.hash(state);
    }
}

impl<T, D> Eq for EndBound<T, D>
where
    T: PartialEq<T>,
    D: Discreteness,
{
}

impl<T, D> PartialEq<EndBound<T, D>> for EndBound<T, D>
where
    T: PartialEq<T>,
    D: Discreteness,
{
    #[inline]
    fn eq(&self, other: &EndBound<T, D>) -> bool {
        self.bound == other.bound
    }
}

impl<T, D> PartialEq<StartBound<T, D>> for EndBound<T, D>
where
    T: PartialEq<T> + Bounded,
    D: Discreteness,
{
    #[inline]
    fn eq(&self, other: &StartBound<T, D>) -> bool {
        use Bound::*;

        match (self.bound(), other.bound()) {
            (Bounded(s), Bounded(t)) => s.eq(t),
            (Bounded(s), Unbounded) => s.eq(&T::min_value()),
            (Unbounded, Bounded(t)) => T::max_value().eq(t),
            (Unbounded, Unbounded) => false,
        }
    }
}

impl<T, D> PartialOrd<EndBound<T, D>> for EndBound<T, D>
where
    T: PartialOrd<T> + Bounded,
    D: Discreteness,
{
    #[inline]
    fn partial_cmp(&self, other: &EndBound<T, D>) -> Option<Ordering> {
        use Bound::*;

        match (self.bound(), other.bound()) {
            (Bounded(s), Bounded(t)) => s.partial_cmp(t),
            (Bounded(s), Unbounded) => s.partial_cmp(&T::max_value()),
            (Unbounded, Bounded(t)) => T::max_value().partial_cmp(t),
            (Unbounded, Unbounded) => Some(Ordering::Equal),
        }
    }
}

impl<T, D> PartialOrd<StartBound<T, D>> for EndBound<T, D>
where
    T: PartialOrd<T> + Bounded,
    D: Discreteness,
{
    #[inline]
    fn partial_cmp(&self, other: &StartBound<T, D>) -> Option<Ordering> {
        use Bound::*;

        match (self.bound(), other.bound()) {
            (Bounded(s), Bounded(t)) => s.partial_cmp(t),
            (Bounded(s), Unbounded) => s.partial_cmp(&T::min_value()),
            (Unbounded, Bounded(t)) => T::max_value().partial_cmp(t),
            (Unbounded, Unbounded) => Some(Ordering::Greater),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use Ordering::*;

    type Bnd = crate::Discrete;
    // type Bnd = crate::NonDiscrete;

    type Lower = StartBound<isize, Bnd>;
    type Upper = EndBound<isize, Bnd>;

    type Lhs = Upper;

    mod vs_end_bound {
        use super::*;

        type Rhs = Upper;

        #[test]
        fn bounded_bounded() {
            fn ordering(s: isize, t: isize) -> Option<Ordering> {
                Lhs::bounded(s).partial_cmp(&Rhs::bounded(t))
            }

            assert_eq!(ordering(1, 1), Some(Equal));
            assert_eq!(ordering(1, 2), Some(Less));
            assert_eq!(ordering(1, 3), Some(Less));

            assert_eq!(ordering(2, 1), Some(Greater));
            assert_eq!(ordering(2, 2), Some(Equal));
            assert_eq!(ordering(2, 3), Some(Less));

            assert_eq!(ordering(3, 1), Some(Greater));
            assert_eq!(ordering(3, 2), Some(Greater));
            assert_eq!(ordering(3, 3), Some(Equal));

            assert_eq!(Lhs::bounded(1), Rhs::bounded(1));
            assert_ne!(Lhs::bounded(1), Rhs::bounded(2));
            assert_ne!(Lhs::bounded(1), Rhs::bounded(3));

            assert_ne!(Lhs::bounded(2), Rhs::bounded(1));
            assert_eq!(Lhs::bounded(2), Rhs::bounded(2));
            assert_ne!(Lhs::bounded(2), Rhs::bounded(3));

            assert_ne!(Lhs::bounded(3), Rhs::bounded(1));
            assert_ne!(Lhs::bounded(3), Rhs::bounded(2));
            assert_eq!(Lhs::bounded(3), Rhs::bounded(3));
        }

        #[test]
        fn bounded_unbounded() {
            fn ordering(s: isize) -> Option<Ordering> {
                Lhs::bounded(s).partial_cmp(&Rhs::unbounded())
            }

            assert_eq!(ordering(isize::MIN), Some(Less));
            assert_eq!(ordering(1), Some(Less));
            assert_eq!(ordering(2), Some(Less));
            assert_eq!(ordering(3), Some(Less));
            assert_eq!(ordering(isize::MAX), Some(Equal));

            assert_ne!(Lhs::bounded(isize::MIN), Rhs::unbounded());
            assert_ne!(Lhs::bounded(1), Rhs::unbounded());
            assert_ne!(Lhs::bounded(2), Rhs::unbounded());
            assert_ne!(Lhs::bounded(3), Rhs::unbounded());
            assert_ne!(Lhs::bounded(isize::MAX), Rhs::unbounded());
        }

        #[test]
        fn unbounded_bounded() {
            fn ordering(t: isize) -> Option<Ordering> {
                Lhs::unbounded().partial_cmp(&Rhs::bounded(t))
            }

            assert_eq!(ordering(isize::MIN), Some(Greater));
            assert_eq!(ordering(1), Some(Greater));
            assert_eq!(ordering(2), Some(Greater));
            assert_eq!(ordering(3), Some(Greater));
            assert_eq!(ordering(isize::MAX), Some(Equal));

            assert_ne!(Rhs::unbounded(), Lhs::bounded(isize::MIN));
            assert_ne!(Rhs::unbounded(), Lhs::bounded(1));
            assert_ne!(Rhs::unbounded(), Lhs::bounded(2));
            assert_ne!(Rhs::unbounded(), Lhs::bounded(3));
            assert_ne!(Rhs::unbounded(), Lhs::bounded(isize::MAX));
        }

        #[test]
        fn unbounded_unbounded() {
            assert_eq!(Lhs::unbounded().partial_cmp(&Rhs::unbounded()), Some(Equal));

            assert_eq!(Lhs::unbounded(), Rhs::unbounded());
        }
    }

    mod vs_start_bound {
        use super::*;

        type Rhs = Lower;

        #[test]
        fn bounded_bounded() {
            fn ordering(s: isize, t: isize) -> Option<Ordering> {
                Lhs::bounded(s).partial_cmp(&Rhs::bounded(t))
            }

            assert_eq!(ordering(1, 1), Some(Equal));
            assert_eq!(ordering(1, 2), Some(Less));
            assert_eq!(ordering(1, 3), Some(Less));

            assert_eq!(ordering(2, 1), Some(Greater));
            assert_eq!(ordering(2, 2), Some(Equal));
            assert_eq!(ordering(2, 3), Some(Less));

            assert_eq!(ordering(3, 1), Some(Greater));
            assert_eq!(ordering(3, 2), Some(Greater));
            assert_eq!(ordering(3, 3), Some(Equal));

            assert_eq!(Lhs::bounded(1), Rhs::bounded(1));
            assert_ne!(Lhs::bounded(1), Rhs::bounded(2));
            assert_ne!(Lhs::bounded(1), Rhs::bounded(3));

            assert_ne!(Lhs::bounded(2), Rhs::bounded(1));
            assert_eq!(Lhs::bounded(2), Rhs::bounded(2));
            assert_ne!(Lhs::bounded(2), Rhs::bounded(3));

            assert_ne!(Lhs::bounded(3), Rhs::bounded(1));
            assert_ne!(Lhs::bounded(3), Rhs::bounded(2));
            assert_eq!(Lhs::bounded(3), Rhs::bounded(3));
        }

        #[test]
        fn bounded_unbounded() {
            fn ordering(s: isize) -> Option<Ordering> {
                Lhs::bounded(s).partial_cmp(&Rhs::unbounded())
            }

            assert_eq!(ordering(isize::MIN), Some(Equal));
            assert_eq!(ordering(1), Some(Greater));
            assert_eq!(ordering(2), Some(Greater));
            assert_eq!(ordering(3), Some(Greater));
            assert_eq!(ordering(isize::MAX), Some(Greater));

            assert_eq!(Lhs::bounded(isize::MIN), Rhs::unbounded());
            assert_ne!(Lhs::bounded(1), Rhs::unbounded());
            assert_ne!(Lhs::bounded(2), Rhs::unbounded());
            assert_ne!(Lhs::bounded(3), Rhs::unbounded());
            assert_ne!(Lhs::bounded(isize::MAX), Rhs::unbounded());
        }

        #[test]
        fn unbounded_bounded() {
            fn ordering(t: isize) -> Option<Ordering> {
                Lhs::unbounded().partial_cmp(&Rhs::bounded(t))
            }

            assert_eq!(ordering(isize::MIN), Some(Greater));
            assert_eq!(ordering(1), Some(Greater));
            assert_eq!(ordering(2), Some(Greater));
            assert_eq!(ordering(3), Some(Greater));
            assert_eq!(ordering(isize::MAX), Some(Equal));

            assert_eq!(Rhs::unbounded(), Lhs::bounded(isize::MIN));
            assert_ne!(Rhs::unbounded(), Lhs::bounded(1));
            assert_ne!(Rhs::unbounded(), Lhs::bounded(2));
            assert_ne!(Rhs::unbounded(), Lhs::bounded(3));
            assert_ne!(Rhs::unbounded(), Lhs::bounded(isize::MAX));
        }

        #[test]
        fn unbounded_unbounded() {
            assert_eq!(
                Lhs::unbounded().partial_cmp(&Rhs::unbounded()),
                Some(Greater)
            );

            assert_ne!(Lhs::unbounded(), Rhs::unbounded());
        }
    }
}
