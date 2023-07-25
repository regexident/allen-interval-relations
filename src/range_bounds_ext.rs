use crate::{Bounded, Discreteness, RangeBounds};

pub trait RangeBoundsExt<T, D>: RangeBounds<T, D>
where
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
    D: Discreteness,
{
    #[inline]
    fn precedes<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        let self_end = self.end_bound();
        let other_start = other.start_bound();
        self_end < other_start
    }

    #[inline]
    fn is_preceded_by<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        other.precedes(self)
    }

    #[inline]
    fn meets<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        let self_end = self.end_bound();
        let other_start = other.start_bound();
        self_end == other_start
    }

    #[inline]
    fn is_met_by<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        other.meets(self)
    }

    #[inline]
    fn overlaps<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        let self_end = self.end_bound();
        let other_start = other.start_bound();
        let other_end = other.end_bound();
        (self_end > other_start) && (self_end < other_end)
    }

    #[inline]
    fn is_overlapped_by<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        other.overlaps(self)
    }

    #[inline]
    fn starts<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        let self_start = self.start_bound();
        let self_end = self.end_bound();
        let other_start = other.start_bound();
        let other_end = other.end_bound();
        (self_start == other_start) && (self_end < other_end)
    }

    #[inline]
    fn is_started_by<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        other.starts(self)
    }

    #[inline]
    fn contains<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        let self_start = self.start_bound();
        let self_end = self.end_bound();
        let other_start = other.start_bound();
        let other_end = other.end_bound();
        (self_start < other_start) && (self_end > other_end)
    }

    #[inline]
    fn is_contained_by<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        other.contains(self)
    }

    #[inline]
    fn finishes<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        let self_start = self.start_bound();
        let self_end = self.end_bound();
        let other_start = other.start_bound();
        let other_end = other.end_bound();
        (self_start > other_start) && (self_end == other_end)
    }

    #[inline]
    fn is_finished_by<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        other.finishes(self)
    }

    #[inline]
    fn equals<R>(&self, other: &R) -> bool
    where
        R: RangeBoundsExt<T, D> + ?Sized,
    {
        let self_start = self.start_bound();
        let self_end = self.end_bound();
        let other_start = other.start_bound();
        let other_end = other.end_bound();
        (self_start == other_start) && (self_end == other_end)
    }
}

impl<X, T, D> RangeBoundsExt<T, D> for X
where
    X: RangeBounds<T, D>,
    T: PartialEq<T> + PartialOrd<T> + Sized + Bounded,
    D: Discreteness,
{
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
//     fn is_contained_by() {
//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌─────────┐
//         // t:                └─────────────────────────────── ─ ─
//         assert!((4..6).is_contained_by(&(3..))); // discrete time-domain
//         assert!((4..=6).is_contained_by(&(3..))); // non-discrete time-domain

//         //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
//         // s:                     ┌─────────┐
//         // t:                └───────────────────┘
//         assert!((4..6).is_contained_by(&(3..7))); // discrete time-domain
//         assert!((4..=6).is_contained_by(&(3..=7))); // non-discrete time-domain
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
