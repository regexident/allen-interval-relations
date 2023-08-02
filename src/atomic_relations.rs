use std::{
    cmp::Ordering,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use crate::{from_ranges::FromRanges, Bounded, IntervalError, IntervalValidator, ValidateInterval};

/// A type describing the possible atomic relations between two intervals (e.g. `s` and `t`).
///
/// Each atomic event `s` can be described using two time points b(s) and e(s).
///
/// See the following paper for more info:
///
/// > Georgala, K., Sherif, M. A., & Ngonga Ngomo, A. C. (2016).
/// > An efficient approach for the generation of Allen relations.
/// > In ECAI 2016 (pp. 948-956). IOS Press.
///
/// To compose the atomic interval relations, we define all possible binary relations between
/// the begin and end points of two event resources `s := (b(s), e(s))` and `t := (b(t), e(t))` as follows:
///
/// Atomic relations between b(s) and b(t):
///
/// - `BB1(s,t)` ⇔ `(b(s) < b(t))`
/// - `BB0(s,t)` ⇔ `(b(s) = b(t))`
/// - `BB−1(s,t)` ⇔ `(b(s) > b(t))` ⇔ `¬(BB1(s,t) ∨ BB0(s,t))`
///
/// Atomic relations between b(s) and e(t):
///
/// - `BE1(s,t)` ⇔ `(b(s) < e(t))`
/// - `BE0(s,t)` ⇔ `(b(s) = e(t))`
/// - `BE−1(s,t)` ⇔ `(b(s) > e(t))` ⇔ `¬(BE1(s,t) ∨ BE0(s,t))`
///
/// Atomic relations between e(s) and b(t):
///
/// - `EB1(s,t)` ⇔ `(e(s) < b(t))`
/// - `EB0(s,t)` ⇔ `(e(s) = b(t))`
/// - `EB−1(s,t)` ⇔ `(e(s) > b(t))` ⇔ `¬(EB1(s,t) ∨ EB0(s,t))`
///
/// Atomic relations between e(s) and e(t):
///
/// - `EE1(s,t)` ⇔ `(e(s) < e(t))`
/// - `EE0(s,t)` ⇔ `(e(s) = e(t))`
/// - `EE−1(s,t)` ⇔ `(e(s) > e(t))` ⇔ `¬(EE1(s,t) ∨ EE0(s,t))`
///
/// The atomic relations map to Rust as follows:
///
/// - `BB1`, `BE1`, `EB1`, `EE1` => `Ordering::Less`
/// - `BB0`, `BE0`, `EB0`, `EE0` => `Ordering::Equal`
/// - `BB-1`, `BE-1`, `EB-1`, `EE-1` => `Ordering::Greater`
///
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct AtomicRelations {
    pub(crate) bb: Ordering,
    pub(crate) be: Ordering,
    pub(crate) eb: Ordering,
    pub(crate) ee: Ordering,
}

impl AtomicRelations {
    fn new<T>(s: (&T, &T), t: (&T, &T)) -> Result<Self, IntervalError>
    where
        T: PartialOrd<T> + Bounded,
    {
        let (s_start, s_end) = s;
        let (t_start, t_end) = t;

        // Check for empty ranges:

        if (s_start >= s_end) || (t_start >= t_end) {
            return Err(IntervalError::EmptyInterval);
        }

        let Some(bb) = s_start.partial_cmp(t_start) else {
            return Err(IntervalError::AmbiguousOrder);
        };
        let Some(be) = s_start.partial_cmp(t_end) else {
            return Err(IntervalError::AmbiguousOrder);
        };
        let Some(eb) = s_end.partial_cmp(t_start) else {
            return Err(IntervalError::AmbiguousOrder);
        };
        let Some(ee) = s_end.partial_cmp(t_end) else {
            return Err(IntervalError::AmbiguousOrder);
        };

        Ok(Self { bb, be, eb, ee })
    }

    /// Returns the ordering between `s`'s start bound and `t`'s start bound.
    #[inline]
    pub fn bb(&self) -> Ordering {
        self.bb
    }

    /// Returns the ordering between `s`'s start bound and `t`'s end bound.
    #[inline]
    pub fn be(&self) -> Ordering {
        self.be
    }

    /// Returns the ordering between `s`'s end bound and `t`'s start bound.
    #[inline]
    pub fn eb(&self) -> Ordering {
        self.eb
    }

    /// Returns the ordering between `s`'s end bound and `t`'s end bound.
    #[inline]
    pub fn ee(&self) -> Ordering {
        self.ee
    }
}

// Lhs: RangeFull

impl FromRanges<RangeFull, RangeFull> for AtomicRelations {
    #[inline]
    fn from_ranges(s: RangeFull, t: RangeFull) -> Result<Self, IntervalError> {
        debug_assert_eq!(s, t);

        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Ok(Self {
            bb: Ordering::Equal,
            be: Ordering::Less,
            eb: Ordering::Greater,
            ee: Ordering::Equal,
        })
    }
}

impl<T> FromRanges<RangeFull, RangeTo<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFull, t: RangeTo<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new(
            (&T::min_value(), &T::max_value()),
            (&T::min_value(), &t.end),
        )
    }
}

impl<T> FromRanges<RangeFull, RangeToInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFull, t: RangeToInclusive<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new(
            (&T::min_value(), &T::max_value()),
            (&T::min_value(), &t.end),
        )
    }
}

impl<T> FromRanges<RangeFull, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFull, t: RangeFrom<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new(
            (&T::min_value(), &T::max_value()),
            (&t.start, &T::max_value()),
        )
    }
}

impl<T> FromRanges<RangeFull, Range<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFull, t: Range<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&T::min_value(), &T::max_value()), (&t.start, &t.end))
    }
}

impl<T> FromRanges<RangeFull, RangeInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFull, t: RangeInclusive<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&T::min_value(), &T::max_value()), (t.start(), t.end()))
    }
}

// Lhs: RangeTo<T>

impl<T> FromRanges<RangeTo<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeTo<T>, t: RangeFull) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new(
            (&T::min_value(), &s.end),
            (&T::min_value(), &T::max_value()),
        )
    }
}

impl<T> FromRanges<RangeTo<T>, RangeTo<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeTo<T>, t: RangeTo<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&T::min_value(), &s.end), (&T::min_value(), &t.end))
    }
}

// `FromRanges<RangeTo<T>, RangeToInclusive<T>>` intentionally omitted.

impl<T> FromRanges<RangeTo<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeTo<T>, t: RangeFrom<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&T::min_value(), &s.end), (&t.start, &T::max_value()))
    }
}

impl<T> FromRanges<RangeTo<T>, Range<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeTo<T>, t: Range<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&T::min_value(), &s.end), (&t.start, &t.end))
    }
}

// `FromRanges<RangeTo<T>, RangeInclusive<T>>` intentionally omitted.

// Lhs: RangeToInclusive<T>

impl<T> FromRanges<RangeToInclusive<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeToInclusive<T>, t: RangeFull) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new(
            (&T::min_value(), &s.end),
            (&T::min_value(), &T::max_value()),
        )
    }
}

// `FromRanges<RangeToInclusive<T>, RangeTo<T>>` intentionally omitted.

impl<T> FromRanges<RangeToInclusive<T>, RangeToInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeToInclusive<T>, t: RangeToInclusive<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&T::min_value(), &s.end), (&T::min_value(), &t.end))
    }
}

impl<T> FromRanges<RangeToInclusive<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeToInclusive<T>, t: RangeFrom<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&T::min_value(), &s.end), (&t.start, &T::max_value()))
    }
}

// `FromRanges<RangeToInclusive<T>, Range<T>>` intentionally omitted.

impl<T> FromRanges<RangeToInclusive<T>, RangeInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeToInclusive<T>, t: RangeInclusive<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&T::min_value(), &s.end), (t.start(), t.end()))
    }
}

// Lhs: RangeFrom<T>

impl<T> FromRanges<RangeFrom<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeFull) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new(
            (&s.start, &T::max_value()),
            (&T::min_value(), &T::max_value()),
        )
    }
}

impl<T> FromRanges<RangeFrom<T>, RangeTo<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeTo<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &T::max_value()), (&T::min_value(), &t.end))
    }
}

impl<T> FromRanges<RangeFrom<T>, RangeToInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeToInclusive<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &T::max_value()), (&T::min_value(), &t.end))
    }
}

impl<T> FromRanges<RangeFrom<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeFrom<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &T::max_value()), (&t.start, &T::max_value()))
    }
}

impl<T> FromRanges<RangeFrom<T>, Range<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: Range<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &T::max_value()), (&t.start, &t.end))
    }
}

impl<T> FromRanges<RangeFrom<T>, RangeInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeInclusive<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &T::max_value()), (t.start(), t.end()))
    }
}

// Lhs: Range<T>

impl<T> FromRanges<Range<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: Range<T>, t: RangeFull) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &s.end), (&T::min_value(), &T::max_value()))
    }
}

impl<T> FromRanges<Range<T>, RangeTo<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: Range<T>, t: RangeTo<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &s.end), (&T::min_value(), &t.end))
    }
}

// `FromRanges<Range<T>, RangeToInclusive<T>>` intentionally omitted.

impl<T> FromRanges<Range<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: Range<T>, t: RangeFrom<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &s.end), (&t.start, &T::max_value()))
    }
}

impl<T> FromRanges<Range<T>, Range<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: Range<T>, t: Range<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((&s.start, &s.end), (&t.start, &t.end))
    }
}

// `FromRanges<Range<T>, RangeInclusive<T>>` intentionally omitted.

// Lhs: RangeInclusive<T>

impl<T> FromRanges<RangeInclusive<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeInclusive<T>, t: RangeFull) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((s.start(), s.end()), (&T::min_value(), &T::max_value()))
    }
}

// `FromRanges<RangeInclusive<T>, RangeTo<T>>` intentionally omitted.

impl<T> FromRanges<RangeInclusive<T>, RangeToInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeInclusive<T>, t: RangeToInclusive<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((s.start(), s.end()), (&T::min_value(), &t.end))
    }
}

impl<T> FromRanges<RangeInclusive<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeInclusive<T>, t: RangeFrom<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((s.start(), s.end()), (&t.start, &T::max_value()))
    }
}

// `FromRanges<RangeInclusive<T>, Range<T>>` intentionally omitted.

impl<T> FromRanges<RangeInclusive<T>, RangeInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeInclusive<T>, t: RangeInclusive<T>) -> Result<Self, IntervalError> {
        IntervalValidator.validate_interval(&s).unwrap();
        IntervalValidator.validate_interval(&t).unwrap();

        Self::new((s.start(), s.end()), (t.start(), t.end()))
    }
}
