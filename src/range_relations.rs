use core::ops::{
    Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use crate::{Bounded, IntervalValidator, ValidateInterval};

/// Extension methods for allen relations.
pub trait RangeRelations<R, T>: RangeBounds<T> + Sized
where
    R: RangeBounds<T> + Sized,
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    /// Returns `true` iff `self` precedes `other.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other:            └────────┘
    /// ```
    #[inline]
    fn precedes(&self, other: &R) -> bool
    where
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        IntervalValidator.validate_interval(self).unwrap();
        IntervalValidator.validate_interval(other).unwrap();

        false
    }

    /// Returns `true` iff `self` is preceded by `other.
    ///
    /// ```plain
    /// self:             ┌────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn is_preceded_by(&self, other: &R) -> bool
    where
        R: RangeRelations<Self, T>,
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        other.precedes(self)
    }

    /// Returns `true` iff `self` meets `other.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other:          └────────┘
    /// ```
    #[inline]
    fn meets(&self, other: &R) -> bool
    where
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        IntervalValidator.validate_interval(self).unwrap();
        IntervalValidator.validate_interval(other).unwrap();

        false
    }

    /// Returns `true` iff `self` is met by `other.
    ///
    /// ```plain
    /// self:           ┌────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn is_met_by(&self, other: &R) -> bool
    where
        R: RangeRelations<Self, T>,
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        other.meets(self)
    }

    /// Returns `true` iff `self` overlaps `other.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other:      └────────┘
    /// ```
    #[inline]
    fn overlaps(&self, other: &R) -> bool
    where
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        IntervalValidator.validate_interval(self).unwrap();
        IntervalValidator.validate_interval(other).unwrap();

        false
    }

    /// Returns `true` iff `self` is overlapped by `other.
    ///
    /// ```plain
    /// self:       ┌────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn is_overlapped_by(&self, other: &R) -> bool
    where
        R: RangeRelations<Self, T>,
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        other.overlaps(self)
    }

    /// Returns `true` iff `self` starts `other.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other: └────────────────┘
    /// ```
    #[inline]
    fn starts(&self, other: &R) -> bool
    where
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        IntervalValidator.validate_interval(self).unwrap();
        IntervalValidator.validate_interval(other).unwrap();

        false
    }

    /// Returns `true` iff `self` is started by `other.
    ///
    /// ```plain
    /// self:  ┌────────────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn is_started_by(&self, other: &R) -> bool
    where
        R: RangeRelations<Self, T>,
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        other.starts(self)
    }

    /// Returns `true` iff `self` contains `other.
    ///
    /// ```plain
    /// self:  ┌────────────────┐
    /// other:     └────────┘
    /// ```
    #[inline]
    fn encloses(&self, other: &R) -> bool
    where
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        IntervalValidator.validate_interval(self).unwrap();
        IntervalValidator.validate_interval(other).unwrap();

        false
    }

    /// Returns `true` iff `self` is contained by `other.
    ///
    /// ```plain
    /// self:      ┌────────┐
    /// other: └────────────────┘
    /// ```
    #[inline]
    fn is_enclosed_by(&self, other: &R) -> bool
    where
        R: RangeRelations<Self, T>,
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        other.encloses(self)
    }

    /// Returns `true` iff `self` finishes `other.
    ///
    /// ```plain
    /// self:          ┌────────┐
    /// other: └────────────────┘
    /// ```
    #[inline]
    fn finishes(&self, other: &R) -> bool
    where
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        IntervalValidator.validate_interval(self).unwrap();
        IntervalValidator.validate_interval(other).unwrap();

        false
    }

    /// Returns `true` iff `self` is finished by `other.
    ///
    /// ```plain
    /// self:  ┌────────────────┐
    /// other:         └────────┘
    /// ```
    #[inline]
    fn is_finished_by(&self, other: &R) -> bool
    where
        R: RangeRelations<Self, T>,
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        other.finishes(self)
    }

    /// Returns `true` iff `self` equals `other.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn equals(&self, other: &R) -> bool
    where
        IntervalValidator: ValidateInterval<Self> + ValidateInterval<R>,
    {
        IntervalValidator.validate_interval(self).unwrap();
        IntervalValidator.validate_interval(other).unwrap();

        false
    }
}

impl<T> RangeRelations<RangeFull, T> for RangeFull
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn equals(&self, _other: &RangeFull) -> bool {
        true
    }
}

impl<T> RangeRelations<RangeTo<T>, T> for RangeFull
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn starts(&self, other: &RangeTo<T>) -> bool {
        T::max_value() < other.end
    }

    #[inline]
    fn equals(&self, other: &RangeTo<T>) -> bool {
        T::max_value() == other.end
    }
}

impl<T> RangeRelations<RangeToInclusive<T>, T> for RangeFull
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn starts(&self, other: &RangeToInclusive<T>) -> bool {
        T::max_value() < other.end
    }

    #[inline]
    fn equals(&self, other: &RangeToInclusive<T>) -> bool {
        T::max_value() == other.end
    }
}

impl<T> RangeRelations<RangeFrom<T>, T> for RangeFull
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn finishes(&self, other: &RangeFrom<T>) -> bool {
        T::min_value() < other.start
    }

    #[inline]
    fn equals(&self, other: &RangeFrom<T>) -> bool {
        T::min_value() == other.start
    }
}

impl<T> RangeRelations<Range<T>, T> for RangeFull
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn encloses(&self, other: &Range<T>) -> bool {
        (T::min_value() < other.start) && (T::max_value() > other.end)
    }

    #[inline]
    fn equals(&self, other: &Range<T>) -> bool {
        (T::min_value() == other.start) && (T::max_value() == other.end)
    }
}

impl<T> RangeRelations<RangeInclusive<T>, T> for RangeFull
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn encloses(&self, other: &RangeInclusive<T>) -> bool {
        (&T::min_value() < other.start()) && (&T::max_value() > other.end())
    }

    fn equals(&self, other: &RangeInclusive<T>) -> bool {
        (&T::min_value() == other.start()) && (&T::max_value() == other.end())
    }
}

impl<T> RangeRelations<RangeFull, T> for RangeTo<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn starts(&self, _other: &RangeFull) -> bool {
        self.end < T::max_value()
    }

    fn equals(&self, _other: &RangeFull) -> bool {
        self.end == T::max_value()
    }
}

impl<T> RangeRelations<RangeTo<T>, T> for RangeTo<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn starts(&self, other: &RangeTo<T>) -> bool {
        self.end < other.end
    }

    fn equals(&self, other: &RangeTo<T>) -> bool {
        self == other
    }
}

impl<T> RangeRelations<RangeFrom<T>, T> for RangeTo<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn precedes(&self, other: &RangeFrom<T>) -> bool {
        self.end < other.start
    }

    fn meets(&self, other: &RangeFrom<T>) -> bool {
        self.end == other.start
    }

    fn overlaps(&self, other: &RangeFrom<T>) -> bool {
        (self.end > other.start) && (self.end < T::max_value())
    }

    fn equals(&self, other: &RangeFrom<T>) -> bool {
        (T::min_value() == other.start) && (self.end == T::max_value())
    }
}

impl<T> RangeRelations<Range<T>, T> for RangeTo<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn precedes(&self, other: &Range<T>) -> bool {
        self.end < other.start
    }

    fn meets(&self, other: &Range<T>) -> bool {
        self.end == other.start
    }

    fn overlaps(&self, other: &Range<T>) -> bool {
        (self.end > other.start) && (self.end < other.end)
    }

    fn equals(&self, other: &Range<T>) -> bool {
        (T::min_value() == other.start) && (self.end == other.end)
    }
}

impl<T> RangeRelations<RangeFull, T> for RangeToInclusive<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn starts(&self, _other: &RangeFull) -> bool {
        self.end < T::max_value()
    }

    fn equals(&self, _other: &RangeFull) -> bool {
        self.end == T::max_value()
    }
}

impl<T> RangeRelations<RangeToInclusive<T>, T> for RangeToInclusive<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn starts(&self, other: &RangeToInclusive<T>) -> bool {
        self.end < other.end
    }

    fn equals(&self, other: &RangeToInclusive<T>) -> bool {
        self == other
    }
}

impl<T> RangeRelations<RangeFrom<T>, T> for RangeToInclusive<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn precedes(&self, other: &RangeFrom<T>) -> bool {
        self.end < other.start
    }

    fn meets(&self, other: &RangeFrom<T>) -> bool {
        self.end == other.start
    }

    fn overlaps(&self, other: &RangeFrom<T>) -> bool {
        (self.end > other.start) && (self.end < T::max_value())
    }

    fn equals(&self, other: &RangeFrom<T>) -> bool {
        (T::min_value() == other.start) && (self.end == T::max_value())
    }
}

impl<T> RangeRelations<RangeInclusive<T>, T> for RangeToInclusive<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    fn precedes(&self, other: &RangeInclusive<T>) -> bool {
        &self.end < other.start()
    }

    fn meets(&self, other: &RangeInclusive<T>) -> bool {
        &self.end == other.start()
    }

    fn overlaps(&self, other: &RangeInclusive<T>) -> bool {
        (&self.end > other.start()) && (&self.end < other.end())
    }

    fn encloses(&self, other: &RangeInclusive<T>) -> bool {
        (&T::min_value() < other.start()) && (&self.end > other.end())
    }

    fn equals(&self, other: &RangeInclusive<T>) -> bool {
        (&T::min_value() == other.start()) && (&self.end == other.end())
    }
}

impl<T> RangeRelations<RangeFull, T> for RangeFrom<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn finishes(&self, _other: &RangeFull) -> bool {
        self.start > T::min_value()
    }

    #[inline]
    fn equals(&self, _other: &RangeFull) -> bool {
        self.start == T::min_value()
    }
}

impl<T> RangeRelations<RangeTo<T>, T> for RangeFrom<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn finishes(&self, other: &RangeTo<T>) -> bool {
        (self.start > T::min_value()) && (T::max_value() == other.end)
    }

    #[inline]
    fn equals(&self, other: &RangeTo<T>) -> bool {
        (self.start == T::min_value()) && (T::max_value() == other.end)
    }
}

impl<T> RangeRelations<RangeToInclusive<T>, T> for RangeFrom<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn finishes(&self, other: &RangeToInclusive<T>) -> bool {
        (self.start > T::min_value()) && (T::max_value() == other.end)
    }

    #[inline]
    fn equals(&self, other: &RangeToInclusive<T>) -> bool {
        (self.start == T::min_value()) && (T::max_value() == other.end)
    }
}

impl<T> RangeRelations<RangeFrom<T>, T> for RangeFrom<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn finishes(&self, other: &RangeFrom<T>) -> bool {
        self.start > other.start
    }

    #[inline]
    fn equals(&self, other: &RangeFrom<T>) -> bool {
        self.start == other.start
    }
}

impl<T> RangeRelations<Range<T>, T> for RangeFrom<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn encloses(&self, other: &Range<T>) -> bool {
        (self.start < other.start) && (T::max_value() > other.end)
    }

    #[inline]
    fn finishes(&self, other: &Range<T>) -> bool {
        (self.start > other.start) && (T::max_value() == other.end)
    }

    #[inline]
    fn equals(&self, other: &Range<T>) -> bool {
        (self.start == other.start) && (T::max_value() == other.end)
    }
}

impl<T> RangeRelations<RangeInclusive<T>, T> for RangeFrom<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn encloses(&self, other: &RangeInclusive<T>) -> bool {
        (&self.start < other.start()) && (&T::max_value() > other.end())
    }

    #[inline]
    fn finishes(&self, other: &RangeInclusive<T>) -> bool {
        (&self.start > other.start()) && (&T::max_value() == other.end())
    }

    #[inline]
    fn equals(&self, other: &RangeInclusive<T>) -> bool {
        (&self.start == other.start()) && (&T::max_value() == other.end())
    }
}

impl<T> RangeRelations<RangeFull, T> for Range<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn meets(&self, _other: &RangeFull) -> bool {
        self.end == T::min_value()
    }

    #[inline]
    fn starts(&self, _other: &RangeFull) -> bool {
        (self.start == T::min_value()) && (self.end < T::max_value())
    }

    #[inline]
    fn finishes(&self, _other: &RangeFull) -> bool {
        (self.start > T::min_value()) && (self.end == T::max_value())
    }

    #[inline]
    fn equals(&self, _other: &RangeFull) -> bool {
        (self.start == T::min_value()) && (self.end == T::max_value())
    }
}

impl<T> RangeRelations<RangeTo<T>, T> for Range<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn meets(&self, _other: &RangeTo<T>) -> bool {
        self.end == T::min_value()
    }

    #[inline]
    fn starts(&self, other: &RangeTo<T>) -> bool {
        (self.start == T::min_value()) && (self.end < other.end)
    }

    #[inline]
    fn finishes(&self, other: &RangeTo<T>) -> bool {
        (self.start > T::min_value()) && (self.end == other.end)
    }

    #[inline]
    fn equals(&self, other: &RangeTo<T>) -> bool {
        (self.start == T::min_value()) && (self.end == other.end)
    }
}

impl<T> RangeRelations<RangeFrom<T>, T> for Range<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn precedes(&self, other: &RangeFrom<T>) -> bool {
        self.end < other.start
    }

    #[inline]
    fn meets(&self, other: &RangeFrom<T>) -> bool {
        self.end == other.start
    }

    #[inline]
    fn overlaps(&self, other: &RangeFrom<T>) -> bool {
        (self.start < other.start) && (other.start < self.end) && (self.end < T::max_value())
    }

    #[inline]
    fn starts(&self, other: &RangeFrom<T>) -> bool {
        (self.start == other.start) && (self.end < T::max_value())
    }

    #[inline]
    fn finishes(&self, other: &RangeFrom<T>) -> bool {
        (self.start > other.start) && (self.end == T::max_value())
    }

    #[inline]
    fn equals(&self, other: &RangeFrom<T>) -> bool {
        (self.start == other.start) && (self.end == T::max_value())
    }
}

impl<T> RangeRelations<Range<T>, T> for Range<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn precedes(&self, other: &Range<T>) -> bool {
        self.end < other.start
    }

    #[inline]
    fn meets(&self, other: &Range<T>) -> bool {
        self.end == other.start
    }

    #[inline]
    fn overlaps(&self, other: &Range<T>) -> bool {
        (self.start < other.start) && (other.start < self.end) && (self.end < other.end)
    }

    #[inline]
    fn starts(&self, other: &Range<T>) -> bool {
        (self.start == other.start) && (self.end < other.end)
    }

    #[inline]
    fn encloses(&self, other: &Range<T>) -> bool {
        (self.start < other.start) && (self.end > other.end)
    }

    #[inline]
    fn finishes(&self, other: &Range<T>) -> bool {
        (self.start > other.start) && (self.end == other.end)
    }

    #[inline]
    fn equals(&self, other: &Range<T>) -> bool {
        (self.start == other.start) && (self.end == other.end)
    }
}

impl<T> RangeRelations<RangeFull, T> for RangeInclusive<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn starts(&self, _other: &RangeFull) -> bool {
        (self.start() == &T::min_value()) && (self.end() < &T::max_value())
    }

    #[inline]
    fn finishes(&self, _other: &RangeFull) -> bool {
        (self.start() > &T::min_value()) && (self.end() == &T::max_value())
    }

    #[inline]
    fn equals(&self, _other: &RangeFull) -> bool {
        (self.start() == &T::min_value()) && (self.end() == &T::max_value())
    }
}

impl<T> RangeRelations<RangeToInclusive<T>, T> for RangeInclusive<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn precedes(&self, _other: &RangeToInclusive<T>) -> bool {
        self.end() < &T::min_value()
    }

    #[inline]
    fn meets(&self, _other: &RangeToInclusive<T>) -> bool {
        self.end() == &T::min_value()
    }

    #[inline]
    fn starts(&self, other: &RangeToInclusive<T>) -> bool {
        (self.start() == &T::min_value()) && (self.end() < &other.end)
    }

    #[inline]
    fn finishes(&self, other: &RangeToInclusive<T>) -> bool {
        (self.start() > &T::min_value()) && (self.end() == &other.end)
    }

    #[inline]
    fn equals(&self, other: &RangeToInclusive<T>) -> bool {
        (self.start() == &T::min_value()) && (self.end() == &other.end)
    }
}

impl<T> RangeRelations<RangeFrom<T>, T> for RangeInclusive<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn precedes(&self, other: &RangeFrom<T>) -> bool {
        self.end() < &other.start
    }

    #[inline]
    fn meets(&self, other: &RangeFrom<T>) -> bool {
        self.end() == &other.start
    }

    #[inline]
    fn overlaps(&self, other: &RangeFrom<T>) -> bool {
        (self.start() < &other.start)
            && (&other.start < self.end())
            && (self.end() < &T::max_value())
    }

    #[inline]
    fn starts(&self, other: &RangeFrom<T>) -> bool {
        (self.start() == &other.start) && (self.end() < &T::max_value())
    }

    #[inline]
    fn finishes(&self, other: &RangeFrom<T>) -> bool {
        (self.start() > &other.start) && (self.end() == &T::max_value())
    }

    #[inline]
    fn equals(&self, other: &RangeFrom<T>) -> bool {
        (self.start() == &other.start) && (self.end() == &T::max_value())
    }
}

impl<T> RangeRelations<RangeInclusive<T>, T> for RangeInclusive<T>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
{
    #[inline]
    fn precedes(&self, other: &RangeInclusive<T>) -> bool {
        self.end() < other.start()
    }

    #[inline]
    fn meets(&self, other: &RangeInclusive<T>) -> bool {
        self.end() == other.start()
    }

    #[inline]
    fn overlaps(&self, other: &RangeInclusive<T>) -> bool {
        (self.start() < other.start()) && (other.start() < self.end()) && (self.end() < other.end())
    }

    #[inline]
    fn starts(&self, other: &RangeInclusive<T>) -> bool {
        (self.start() == other.start()) && (self.end() < other.end())
    }

    #[inline]
    fn encloses(&self, other: &RangeInclusive<T>) -> bool {
        (self.start() < other.start()) && (self.end() > other.end())
    }

    #[inline]
    fn finishes(&self, other: &RangeInclusive<T>) -> bool {
        (self.start() > other.start()) && (self.end() == other.end())
    }

    #[inline]
    fn equals(&self, other: &RangeInclusive<T>) -> bool {
        (self.start() == other.start()) && (self.end() == other.end())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{Discrete, NonDiscrete};

//     use super::*;

//     #[test]
//     fn precedes() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s: ─ ─ ────────────────┐
//         // t:                          └───────────────────── ─ ─
//         assert!((..4).precedes(&(5..))); // discrete time-domain
//         assert!((..=4).precedes(&(5..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s: ─ ─ ────────────────┐
//         // t:                          └──────────────┘
//         assert!((..4).precedes(&(5..8))); // discrete time-domain
//         assert!((..=4).precedes(&(5..=8))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:      ┌──────────────┐
//         // t:                          └───────────────────── ─ ─
//         assert!((1..4).precedes(&(5..))); // discrete time-domain
//         assert!((1..=4).precedes(&(5..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:      ┌──────────────┐
//         // t:                          └──────────────┘
//         assert!((1..4).precedes(&(5..8))); // discrete time-domain
//         assert!((1..=4).precedes(&(5..=8))); // non-discrete time-domain
//     }

//     #[test]
//     fn is_preceded_by() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌───────────────────── ─ ─
//         // t: ─ ─ ────────────────┘
//         assert!((5..).is_preceded_by(&(..4))); // discrete time-domain
//         assert!((5..).is_preceded_by(&(..=4))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌──────────────┐
//         // t: ─ ─ ────────────────┘
//         assert!((5..8).is_preceded_by(&(..4))); // discrete time-domain
//         assert!((5..=8).is_preceded_by(&(..=4))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌───────────────────── ─ ─
//         // t:      └──────────────┘
//         assert!((5..).is_preceded_by(&(1..4))); // discrete time-domain
//         assert!((5..).is_preceded_by(&(1..=4))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌──────────────┐
//         // t:      └──────────────┘
//         assert!((5..8).is_preceded_by(&(1..4))); // discrete time-domain
//         assert!((5..=8).is_preceded_by(&(1..=4))); // non-discrete time-domain
//     }

//     #[test]
//     fn meets() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s: ─ ─ ─────────────────────┐
//         // t:                          └───────────────────── ─ ─
//         assert!((..5).meets(&(5..))); // discrete time-domain
//         assert!((..=5).meets(&(5..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s: ─ ─ ─────────────────────┐
//         // t:                          └──────────────┘
//         assert!((..5).meets(&(5..8))); // discrete time-domain
//         assert!((..=5).meets(&(5..=8))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:           ┌──────────────┐
//         // t:                          └───────────────────── ─ ─
//         assert!((2..5).meets(&(5..))); // discrete time-domain
//         assert!((2..=5).meets(&(5..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:           ┌──────────────┐
//         // t:                          └──────────────┘
//         assert!((2..5).meets(&(5..8))); // discrete time-domain
//         assert!((2..=5).meets(&(5..=8))); // non-discrete time-domain
//     }

//     #[test]
//     fn is_met_by() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌───────────────────── ─ ─
//         // t: ─ ─ ─────────────────────┘
//         assert!((5..).is_met_by(&(..5))); // discrete time-domain
//         assert!((5..).is_met_by(&(..=5))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌──────────────┐
//         // t: ─ ─ ─────────────────────┘
//         assert!((5..8).is_met_by(&(..5))); // discrete time-domain
//         assert!((5..=8).is_met_by(&(..=5))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌───────────────────── ─ ─
//         // t:           └──────────────┘
//         assert!((5..).is_met_by(&(2..5))); // discrete time-domain
//         assert!((5..).is_met_by(&(2..=5))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌──────────────┐
//         // t:           └──────────────┘
//         assert!((5..8).is_met_by(&(2..5))); // discrete time-domain
//         assert!((5..=8).is_met_by(&(2..=5))); // non-discrete time-domain
//     }

//     #[test]
//     fn overlaps() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s: ─ ─ ──────────────────────────┐
//         // t:                     └────────────────────────── ─ ─
//         assert!((..6).overlaps(&(4..))); // discrete time-domain
//         assert!((..=6).overlaps(&(4..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s: ─ ─ ──────────────────────────┐
//         // t:                     └──────────────┘
//         assert!((..6).overlaps(&(4..7))); // discrete time-domain
//         assert!((..=6).overlaps(&(4..=7))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                ┌──────────────┐
//         // t:                     └────────────────────────── ─ ─
//         assert!((3..6).overlaps(&(4..))); // discrete time-domain
//         assert!((3..=6).overlaps(&(4..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                ┌──────────────┐
//         // t:                     └──────────────┘
//         assert!((3..6).overlaps(&(4..7))); // discrete time-domain
//         assert!((3..=6).overlaps(&(4..=7))); // non-discrete time-domain
//     }

//     #[test]
//     fn is_overlapped_by() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌────────────────────────── ─ ─
//         // t: ─ ─ ──────────────────────────┘
//         assert!((4..).is_overlapped_by(&(..6))); // discrete time-domain
//         assert!((4..).is_overlapped_by(&(..=6))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌──────────────┐
//         // t: ─ ─ ──────────────────────────┘
//         assert!((4..7).is_overlapped_by(&(..6))); // discrete time-domain
//         assert!((4..=7).is_overlapped_by(&(..=6))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌────────────────────────── ─ ─
//         // t:                └──────────────┘
//         assert!((4..).is_overlapped_by(&(3..6))); // discrete time-domain
//         assert!((4..).is_overlapped_by(&(3..=6))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌──────────────┐
//         // t:                └──────────────┘
//         assert!((4..7).is_overlapped_by(&(3..6))); // discrete time-domain
//         assert!((4..=7).is_overlapped_by(&(3..=6))); // non-discrete time-domain
//     }

//     #[test]
//     fn starts() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌──────────────┐
//         // t:                     └────────────────────────── ─ ─
//         assert!((4..7).starts(&(4..))); // discrete time-domain
//         assert!((4..=7).starts(&(4..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌──────────────┐
//         // t:                     └───────────────────┘
//         assert!((4..7).starts(&(4..8))); // discrete time-domain
//         assert!((4..=7).starts(&(4..=8))); // non-discrete time-domain
//     }

//     #[test]
//     fn is_started_by() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌────────────────────────── ─ ─
//         // t:                     └──────────────┘
//         assert!((4..).is_started_by(&(4..7))); // discrete time-domain
//         assert!((4..).is_started_by(&(4..=7))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌───────────────────┐
//         // t:                     └──────────────┘
//         assert!((4..8).is_started_by(&(4..7))); // discrete time-domain
//         assert!((4..=8).is_started_by(&(4..=7))); // non-discrete time-domain
//     }

//     #[test]
//     fn contains() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                ┌─────────────────────────────── ─ ─
//         // t:                     └─────────┘
//         assert!((3..).contains(&(4..6))); // discrete time-domain
//         assert!((3..).contains(&(4..=6))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                ┌───────────────────┐
//         // t:                     └─────────┘
//         assert!((3..7).contains(&(4..6))); // discrete time-domain
//         assert!((3..=7).contains(&(4..=6))); // non-discrete time-domain
//     }

//     #[test]
//     fn is_enclosed_by() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌─────────┐
//         // t:                └─────────────────────────────── ─ ─
//         assert!((4..6).is_enclosed_by(&(3..))); // discrete time-domain
//         assert!((4..=6).is_enclosed_by(&(3..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌─────────┐
//         // t:                └───────────────────┘
//         assert!((4..6).is_enclosed_by(&(3..7))); // discrete time-domain
//         assert!((4..=6).is_enclosed_by(&(3..=7))); // non-discrete time-domain
//     }

//     #[test]
//     fn finishes() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌──────────────┐
//         // t: ─ ─ ───────────────────────────────┘
//         assert!((4..7).finishes(&(..7))); // discrete time-domain
//         assert!((4..=7).finishes(&(..=7))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌──────────────┐
//         // t:                └───────────────────┘
//         assert!((4..7).finishes(&(3..7))); // discrete time-domain
//         assert!((4..=7).finishes(&(3..=7))); // non-discrete time-domain
//     }

//     #[test]
//     fn is_finished_by() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // t: ─ ─ ───────────────────────────────┐
//         // t:                     └──────────────┘
//         assert!((..7).is_finished_by(&(4..7))); // discrete time-domain
//         assert!((..=7).is_finished_by(&(4..=7))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                ┌───────────────────┐
//         // t:                     └──────────────┘
//         assert!((3..7).is_finished_by(&(4..7))); // discrete time-domain
//         assert!((3..=7).is_finished_by(&(4..=7))); // non-discrete time-domain
//     }

//     #[test]
//     fn equals() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s: ─ ─ ─────────────────────────────────────────── ─ ─
//         // t: ─ ─ ─────────────────────────────────────────── ─ ─
//         assert!((..).equals(&(..))); // discrete time-domain
//         assert!((..).equals(&(..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                          ┌───────────────────── ─ ─
//         // t:                          └───────────────────── ─ ─
//         assert!((5..).equals(&(5..))); // discrete time-domain
//         assert!((5..).equals(&(5..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s: ─ ─ ─────────────────────┐
//         // t: ─ ─ ─────────────────────┘
//         assert!((..5).equals(&(..5))); // discrete time-domain
//         assert!((..=5).equals(&(..=5))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌─────────┐
//         // t:                     └─────────┘
//         assert!((4..6).equals(&(4..6))); // discrete time-domain
//         assert!((4..=6).equals(&(4..=6))); // non-discrete time-domain
//     }
// }
