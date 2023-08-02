use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::Bounded;

/// Error type specific to Allen's interval algebra.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntervalError {
    /// Empty intervals are invalid with respect to Allen's interval algebra.
    EmptyInterval,
    /// Could not obtain total order.
    AmbiguousOrder,
}

/// A type for validating intervals with respect to Allen's interval algebra.
pub struct IntervalValidator;

/// A trait for validating intervals with respect to Allen's interval algebra.
pub trait ValidateInterval<T> {
    /// Validates an interval with respect to Allen's interval algebra.
    ///
    /// Empty intervals are invalid as their relations are undefined.
    fn validate_interval(&self, interval: &T) -> Result<(), IntervalError>;
}

impl ValidateInterval<RangeFull> for IntervalValidator {
    #[inline]
    fn validate_interval(&self, _interval: &RangeFull) -> Result<(), IntervalError> {
        Ok(())
    }
}

impl<U> ValidateInterval<RangeTo<U>> for IntervalValidator
where
    U: PartialOrd + Bounded,
{
    #[inline]
    fn validate_interval(&self, interval: &RangeTo<U>) -> Result<(), IntervalError> {
        if interval.end <= U::min_value() {
            Err(IntervalError::EmptyInterval)
        } else {
            Ok(())
        }
    }
}

impl<U> ValidateInterval<RangeToInclusive<U>> for IntervalValidator
where
    U: PartialOrd + Bounded,
{
    #[inline]
    fn validate_interval(&self, interval: &RangeToInclusive<U>) -> Result<(), IntervalError> {
        if interval.end <= U::min_value() {
            Err(IntervalError::EmptyInterval)
        } else {
            Ok(())
        }
    }
}

impl<U> ValidateInterval<RangeFrom<U>> for IntervalValidator
where
    U: PartialOrd + Bounded,
{
    #[inline]
    fn validate_interval(&self, interval: &RangeFrom<U>) -> Result<(), IntervalError> {
        if interval.start >= U::max_value() {
            Err(IntervalError::EmptyInterval)
        } else {
            Ok(())
        }
    }
}

impl<U> ValidateInterval<Range<U>> for IntervalValidator
where
    U: PartialOrd + Bounded,
{
    #[inline]
    fn validate_interval(&self, interval: &Range<U>) -> Result<(), IntervalError> {
        let (start, end) = (&interval.start, &interval.end);
        let (min, max) = (&U::min_value(), &U::max_value());

        if (start >= end) || (start >= max) || (end <= min) {
            Err(IntervalError::EmptyInterval)
        } else {
            Ok(())
        }
    }
}

impl<U> ValidateInterval<RangeInclusive<U>> for IntervalValidator
where
    U: PartialOrd + Bounded,
{
    #[inline]
    fn validate_interval(&self, interval: &RangeInclusive<U>) -> Result<(), IntervalError> {
        let (start, end) = (interval.start(), interval.end());
        let (min, max) = (&U::min_value(), &U::max_value());

        if (start >= end) || (start >= max) || (end <= min) {
            Err(IntervalError::EmptyInterval)
        } else {
            Ok(())
        }
    }
}
