use std::{
    cmp::Ordering,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use crate::{from_ranges::FromRanges, Bounded};

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
pub struct AtomicRelations {
    pub(crate) bb: Ordering,
    pub(crate) be: Ordering,
    pub(crate) eb: Ordering,
    pub(crate) ee: Ordering,
}

impl AtomicRelations {
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
    fn from_ranges(s: RangeFull, t: RangeFull) -> Option<Self> {
        debug_assert_eq!(s, t);

        Some(Self {
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
    fn from_ranges(_s: RangeFull, t: RangeTo<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&T::min_value())?;
        let be = T::min_value().partial_cmp(&t.end)?;
        let eb = T::max_value().partial_cmp(&T::min_value())?;
        let ee = T::max_value().partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFull, RangeToInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(_s: RangeFull, t: RangeToInclusive<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&T::min_value())?;
        let be = T::min_value().partial_cmp(&t.end)?;
        let eb = T::max_value().partial_cmp(&T::min_value())?;
        let ee = T::max_value().partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFull, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(_s: RangeFull, t: RangeFrom<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&t.start)?;
        let be = T::min_value().partial_cmp(&T::max_value())?;
        let eb = T::max_value().partial_cmp(&t.start)?;
        let ee = T::max_value().partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFull, Range<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(_s: RangeFull, t: Range<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&t.start)?;
        let be = T::min_value().partial_cmp(&t.end)?;
        let eb = T::max_value().partial_cmp(&t.start)?;
        let ee = T::max_value().partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFull, RangeInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(_s: RangeFull, t: RangeInclusive<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(t.start())?;
        let be = T::min_value().partial_cmp(t.end())?;
        let eb = T::max_value().partial_cmp(t.start())?;
        let ee = T::max_value().partial_cmp(t.end())?;

        Some(Self { bb, be, eb, ee })
    }
}

// Lhs: RangeTo<T>

impl<T> FromRanges<RangeTo<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeTo<T>, _t: RangeFull) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&T::min_value())?;
        let be = T::min_value().partial_cmp(&T::max_value())?;
        let eb = s.end.partial_cmp(&T::min_value())?;
        let ee = s.end.partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeTo<T>, RangeTo<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeTo<T>, t: RangeTo<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&T::min_value())?;
        let be = T::min_value().partial_cmp(&t.end)?;
        let eb = s.end.partial_cmp(&T::min_value())?;
        let ee = s.end.partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

// `FromRanges<RangeTo<T>, RangeToInclusive<T>>` intentionally omitted.

impl<T> FromRanges<RangeTo<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeTo<T>, t: RangeFrom<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&t.start)?;
        let be = T::min_value().partial_cmp(&T::max_value())?;
        let eb = s.end.partial_cmp(&t.start)?;
        let ee = s.end.partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeTo<T>, Range<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeTo<T>, t: Range<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&t.start)?;
        let be = T::min_value().partial_cmp(&t.end)?;
        let eb = s.end.partial_cmp(&t.start)?;
        let ee = s.end.partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

// `FromRanges<RangeTo<T>, RangeInclusive<T>>` intentionally omitted.

// Lhs: RangeToInclusive<T>

impl<T> FromRanges<RangeToInclusive<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeToInclusive<T>, _t: RangeFull) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&T::min_value())?;
        let be = T::min_value().partial_cmp(&T::max_value())?;
        let eb = s.end.partial_cmp(&T::min_value())?;
        let ee = s.end.partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

// `FromRanges<RangeToInclusive<T>, RangeTo<T>>` intentionally omitted.

impl<T> FromRanges<RangeToInclusive<T>, RangeToInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeToInclusive<T>, t: RangeToInclusive<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&T::min_value())?;
        let be = T::min_value().partial_cmp(&t.end)?;
        let eb = s.end.partial_cmp(&T::min_value())?;
        let ee = s.end.partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeToInclusive<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeToInclusive<T>, t: RangeFrom<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(&t.start)?;
        let be = T::min_value().partial_cmp(&T::max_value())?;
        let eb = s.end.partial_cmp(&t.start)?;
        let ee = s.end.partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

// `FromRanges<RangeToInclusive<T>, Range<T>>` intentionally omitted.

impl<T> FromRanges<RangeToInclusive<T>, RangeInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeToInclusive<T>, t: RangeInclusive<T>) -> Option<Self> {
        let bb = T::min_value().partial_cmp(t.start())?;
        let be = T::min_value().partial_cmp(t.end())?;
        let eb = s.end.partial_cmp(t.start())?;
        let ee = s.end.partial_cmp(t.end())?;

        Some(Self { bb, be, eb, ee })
    }
}

// Lhs: RangeFrom<T>

impl<T> FromRanges<RangeFrom<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, _t: RangeFull) -> Option<Self> {
        let bb = s.start.partial_cmp(&T::min_value())?;
        let be = s.start.partial_cmp(&T::max_value())?;
        let eb = T::max_value().partial_cmp(&T::min_value())?;
        let ee = T::max_value().partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFrom<T>, RangeTo<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeTo<T>) -> Option<Self> {
        let bb = s.start.partial_cmp(&T::min_value())?;
        let be = s.start.partial_cmp(&t.end)?;
        let eb = T::max_value().partial_cmp(&T::min_value())?;
        let ee = T::max_value().partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFrom<T>, RangeToInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeToInclusive<T>) -> Option<Self> {
        let bb = s.start.partial_cmp(&T::min_value())?;
        let be = s.start.partial_cmp(&t.end)?;
        let eb = T::max_value().partial_cmp(&T::min_value())?;
        let ee = T::max_value().partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFrom<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeFrom<T>) -> Option<Self> {
        let bb = s.start.partial_cmp(&t.start)?;
        let be = s.start.partial_cmp(&T::max_value())?;
        let eb = T::max_value().partial_cmp(&t.start)?;
        let ee = T::max_value().partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFrom<T>, Range<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: Range<T>) -> Option<Self> {
        let bb = s.start.partial_cmp(&t.start)?;
        let be = s.start.partial_cmp(&t.end)?;
        let eb = T::max_value().partial_cmp(&t.start)?;
        let ee = T::max_value().partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeFrom<T>, RangeInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeFrom<T>, t: RangeInclusive<T>) -> Option<Self> {
        let bb = s.start.partial_cmp(t.start())?;
        let be = s.start.partial_cmp(t.end())?;
        let eb = T::max_value().partial_cmp(t.start())?;
        let ee = T::max_value().partial_cmp(t.end())?;

        Some(Self { bb, be, eb, ee })
    }
}

// Lhs: Range<T>

impl<T> FromRanges<Range<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: Range<T>, _t: RangeFull) -> Option<Self> {
        let bb = s.start.partial_cmp(&T::min_value())?;
        let be = s.start.partial_cmp(&T::max_value())?;
        let eb = s.end.partial_cmp(&T::min_value())?;
        let ee = s.end.partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<Range<T>, RangeTo<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: Range<T>, t: RangeTo<T>) -> Option<Self> {
        let bb = s.start.partial_cmp(&T::min_value())?;
        let be = s.start.partial_cmp(&t.end)?;
        let eb = s.end.partial_cmp(&T::min_value())?;
        let ee = s.end.partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

// `FromRanges<Range<T>, RangeToInclusive<T>>` intentionally omitted.

impl<T> FromRanges<Range<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: Range<T>, t: RangeFrom<T>) -> Option<Self> {
        let bb = s.start.partial_cmp(&t.start)?;
        let be = s.start.partial_cmp(&T::max_value())?;
        let eb = s.end.partial_cmp(&t.start)?;
        let ee = s.end.partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<Range<T>, Range<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: Range<T>, t: Range<T>) -> Option<Self> {
        let bb = s.start.partial_cmp(&t.start)?;
        let be = s.start.partial_cmp(&t.end)?;
        let eb = s.end.partial_cmp(&t.start)?;
        let ee = s.end.partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

// `FromRanges<Range<T>, RangeInclusive<T>>` intentionally omitted.

// Lhs: RangeInclusive<T>

impl<T> FromRanges<RangeInclusive<T>, RangeFull> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeInclusive<T>, _t: RangeFull) -> Option<Self> {
        let bb = s.start().partial_cmp(&T::min_value())?;
        let be = s.start().partial_cmp(&T::max_value())?;
        let eb = s.end().partial_cmp(&T::min_value())?;
        let ee = s.end().partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

// `FromRanges<RangeInclusive<T>, RangeTo<T>>` intentionally omitted.

impl<T> FromRanges<RangeInclusive<T>, RangeToInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeInclusive<T>, t: RangeToInclusive<T>) -> Option<Self> {
        let bb = s.start().partial_cmp(&T::min_value())?;
        let be = s.start().partial_cmp(&t.end)?;
        let eb = s.end().partial_cmp(&T::min_value())?;
        let ee = s.end().partial_cmp(&t.end)?;

        Some(Self { bb, be, eb, ee })
    }
}

impl<T> FromRanges<RangeInclusive<T>, RangeFrom<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeInclusive<T>, t: RangeFrom<T>) -> Option<Self> {
        let bb = s.start().partial_cmp(&t.start)?;
        let be = s.start().partial_cmp(&T::max_value())?;
        let eb = s.end().partial_cmp(&t.start)?;
        let ee = s.end().partial_cmp(&T::max_value())?;

        Some(Self { bb, be, eb, ee })
    }
}

// `FromRanges<RangeInclusive<T>, Range<T>>` intentionally omitted.

impl<T> FromRanges<RangeInclusive<T>, RangeInclusive<T>> for AtomicRelations
where
    T: PartialOrd<T> + Bounded,
{
    #[inline]
    fn from_ranges(s: RangeInclusive<T>, t: RangeInclusive<T>) -> Option<Self> {
        let bb = s.start().partial_cmp(t.start())?;
        let be = s.start().partial_cmp(t.end())?;
        let eb = s.end().partial_cmp(t.start())?;
        let ee = s.end().partial_cmp(t.end())?;

        Some(Self { bb, be, eb, ee })
    }
}
