use core::ops::RangeFull;

use crate::{
    Contains, Equals, Finishes, Interval, IntervalFrom, IntervalTo, Meets, NonEmpty, Overlaps,
    Precedes, Starts,
};

/// An unbounded interval (`..`).
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct IntervalFull;

impl From<RangeFull> for IntervalFull {
    fn from(_value: RangeFull) -> Self {
        Self
    }
}

impl From<IntervalFull> for RangeFull {
    fn from(_value: IntervalFull) -> Self {
        Self
    }
}

// IntervalFull vs. IntervalFull

impl Precedes<NonEmpty<IntervalFull>> for NonEmpty<IntervalFull> {}

impl Meets<NonEmpty<IntervalFull>> for NonEmpty<IntervalFull> {}

impl Overlaps<NonEmpty<IntervalFull>> for NonEmpty<IntervalFull> {}

impl Starts<NonEmpty<IntervalFull>> for NonEmpty<IntervalFull> {}

impl Contains<NonEmpty<IntervalFull>> for NonEmpty<IntervalFull> {}

impl Finishes<NonEmpty<IntervalFull>> for NonEmpty<IntervalFull> {}

impl Equals<NonEmpty<IntervalFull>> for NonEmpty<IntervalFull> {
    #[inline]
    fn equals(&self, _other: &NonEmpty<IntervalFull>) -> bool {
        true
    }
}

// IntervalFull vs. IntervalTo

impl<T> Precedes<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFull> {}

impl<T> Meets<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFull> {}

impl<T> Overlaps<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFull> {}

impl<T> Starts<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFull> {}

impl<T> Contains<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFull> {}

impl<T> Finishes<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFull> {}

impl<T> Equals<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFull> {}

// IntervalFull vs. IntervalFrom

impl<T> Precedes<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFull> {}

impl<T> Meets<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFull> {}

impl<T> Overlaps<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFull> {}

impl<T> Starts<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFull> {}

impl<T> Contains<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFull> {}

impl<T> Finishes<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFull> {}

impl<T> Equals<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFull> {}

// IntervalFull vs. Interval

impl<T> Precedes<NonEmpty<Interval<T>>> for NonEmpty<IntervalFull> {}

impl<T> Meets<NonEmpty<Interval<T>>> for NonEmpty<IntervalFull> {}

impl<T> Overlaps<NonEmpty<Interval<T>>> for NonEmpty<IntervalFull> {}

impl<T> Starts<NonEmpty<Interval<T>>> for NonEmpty<IntervalFull> {}

impl<T> Contains<NonEmpty<Interval<T>>> for NonEmpty<IntervalFull> {
    fn contains(&self, _other: &NonEmpty<Interval<T>>) -> bool {
        true
    }
}

impl<T> Finishes<NonEmpty<Interval<T>>> for NonEmpty<IntervalFull> {}

impl<T> Equals<NonEmpty<Interval<T>>> for NonEmpty<IntervalFull> {}
