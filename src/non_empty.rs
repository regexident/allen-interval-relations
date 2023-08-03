use core::cmp::Ordering;

use crate::{Interval, IntervalError, IntervalFrom, IntervalFull, IntervalTo};

/// An interval that is known not to be empty.
///
/// # Layout
///
/// `NonEmpty<T>` is guaranteed to have the same layout and bit validity as `T`
/// with the exception that non-empty instances are valid.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct NonEmpty<T>(pub(crate) T);

impl<T> NonEmpty<T> {
    /// Creates a non-empty without checking whether the value is non-empty.
    /// This results in undefined behavior if the value is empty.
    ///
    /// # Safety
    ///
    /// The value must not be empty.
    #[inline]
    pub unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }
}

impl<T> TryFrom<Interval<T>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    type Error = IntervalError;

    fn try_from(value: Interval<T>) -> Result<Self, Self::Error> {
        match value.start.partial_cmp(&value.end) {
            Some(Ordering::Less) => Ok(Self(value)),
            Some(Ordering::Equal) => Err(IntervalError::EmptyInterval),
            Some(Ordering::Greater) => Err(IntervalError::EmptyInterval),
            None => Err(IntervalError::AmbiguousOrder),
        }
    }
}

impl<T> From<IntervalTo<T>> for NonEmpty<IntervalTo<T>> {
    #[inline]
    fn from(value: IntervalTo<T>) -> Self {
        Self(value)
    }
}

impl<T> From<IntervalFrom<T>> for NonEmpty<IntervalFrom<T>> {
    #[inline]
    fn from(value: IntervalFrom<T>) -> Self {
        Self(value)
    }
}

impl From<IntervalFull> for NonEmpty<IntervalFull> {
    #[inline]
    fn from(value: IntervalFull) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let min = isize::min_value();
        let mid = 0;
        let max = isize::max_value();

        assert_eq!(
            NonEmpty::try_from(Interval {
                start: min,
                end: min
            }),
            Err(IntervalError::EmptyInterval)
        );
        assert_eq!(
            NonEmpty::try_from(Interval {
                start: mid,
                end: mid
            }),
            Err(IntervalError::EmptyInterval)
        );
        assert_eq!(
            NonEmpty::try_from(Interval {
                start: max,
                end: max
            }),
            Err(IntervalError::EmptyInterval)
        );

        assert_eq!(
            NonEmpty::try_from(Interval {
                start: max,
                end: mid
            }),
            Err(IntervalError::EmptyInterval)
        );
        assert_eq!(
            NonEmpty::try_from(Interval {
                start: mid,
                end: min
            }),
            Err(IntervalError::EmptyInterval)
        );

        assert_eq!(
            NonEmpty::try_from(Interval {
                start: min,
                end: mid
            }),
            Ok(NonEmpty(Interval {
                start: min,
                end: mid
            }))
        );
        assert_eq!(
            NonEmpty::try_from(Interval {
                start: mid,
                end: max
            }),
            Ok(NonEmpty(Interval {
                start: mid,
                end: max
            }))
        );
    }
}
