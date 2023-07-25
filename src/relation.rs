use std::cmp::Ordering;

use crate::{AtomicRelations, Bounded, Discrete, Discreteness, NonDiscrete, RangeBounds};

/// A type describing the possible relations between two intervals (e.g. `s` and `t`).
///
/// Each of Allen’s relations can be reduced to a boolean combination of
/// a subset of atomic relations (see [AtomicRelations]) as follows:
///
/// - `PRECEDES(s,t)`:
///   - ⇔ `{ EB1(s,t) }`
/// - `IS_PRECEDED_BY(s,t)`:
///   - ⇔ `{ BE−1 }`
///   - ⇔ `¬(BE1(s,t) ∨ BE0(s,t))`
/// - `MEETS(s,t)`
///   - ⇔ `{ EB0(s,t) }`
/// - `IS_MET_BY(s,t)`
///   - ⇔ `{ BE0(s,t) }`
/// - `FINISHES(s,t)`
///   - ⇔ `{ EE0(s,t) ∧ BB−1(s,t) }`
///   - ⇔ `{ EE0(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t)) }`
/// - `IS_FINISHED_BY(s,t)`
///   - ⇔ `{ BB1(s,t) ∧ EE0(s,t) }`
/// - `STARTS(s,t)`
///   - ⇔ `{ BB0(s,t) ∧ EE1(s,t) }`
/// - `IS_STARTED_BY(s,t)`
///   - ⇔ `{ BB0(s,t) ∧ EE−1(s,t) }`
///   - ⇔ `{ BB0(s,t) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }`
/// - `CONTAINS(s,t)`
///   - ⇔ `{ BB1(s,t) ∧ EE−1(s,t) }`
///   - ⇔ `{ BB1(s,t) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }`
/// - `IS_CONTAINED_BY(s,t)`
///   - ⇔ `{ EE1(s,t) ∧ BB−1(s,t) }`
///   - ⇔ `{ EE1(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t)) }`
/// - `EQUALS(s,t)`
///   - ⇔ `{ BB0(s,t) ∧ EE0(s,t) }`
/// - `OVERLAPS(s,t)`
///   - ⇔ `{ BB1(s,t) ∧ EB−1(s,t) ∧ EE1(s,t) }`
///   - ⇔ `{ (BB1(s,t) ∧ EE1(s,t)) ∧ ¬(EB0(s,t) ∨ EB1(s,t)) }`
/// - `IS_OVERLAPPED_BY(s,t)`
///   - ⇔ `{ BB−1(s,t) ∧ BE1(s,t) ∧ EE−1(s,t) }`
///   - ⇔ `{ (BE1(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t))) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }`
///
/// The relations are comparable (via `Ord`) by the degree to which `s` begins before `t` and then within that by the degree to which `s` ends before `t`.
///
/// Six pairs of the relations are converses. For example, the converse of "s precedes t" is "t is preceded by s";
/// whenever the first relation is true, its converse is true also. The thirteenth, "s equals t", is its own converse
#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Relation {
    /// ```plain
    /// s: ┌────────┐
    /// t:            └────────┘
    /// ```
    Precedes,
    /// ```plain
    /// s: ┌────────┐
    /// t:          └────────┘
    /// ```
    Meets,
    /// ```plain
    /// s: ┌────────┐
    /// t:      └────────┘
    /// ```
    Overlaps,
    /// ```plain
    /// s: ┌────────────────┐
    /// t:         └────────┘
    /// ```
    IsFinishedBy,
    /// ```plain
    /// s: ┌────────────────┐
    /// t:     └────────┘
    /// ```
    Contains,
    /// ```plain
    /// s: ┌────────┐
    /// t: └────────────────┘
    /// ```
    Starts,
    /// ```plain
    /// s: ┌────────┐
    /// t: └────────┘
    /// ```
    Equals,
    /// ```plain
    /// s: ┌────────────────┐
    /// t: └────────┘
    /// ```
    IsStartedBy,
    /// ```plain
    /// s:     ┌────────┐
    /// t: └────────────────┘
    /// ```
    IsContainedBy,
    /// ```plain
    /// s:         ┌────────┐
    /// t: └────────────────┘
    /// ```
    Finishes,
    /// ```plain
    /// s:      ┌────────┐
    /// t: └────────┘
    /// ```
    IsOverlappedBy,
    /// ```plain
    /// s:          ┌────────┐
    /// t: └────────┘
    /// ```
    IsMetBy,
    /// ```plain
    /// s:            ┌────────┐
    /// t: └────────┘
    /// ```
    IsPrecededBy,
}

impl Ord for Relation {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl PartialOrd for Relation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<AtomicRelations> for Relation {
    #[inline]
    fn from(atomics: AtomicRelations) -> Self {
        use Ordering::*;

        let AtomicRelations { bb, be, eb, ee } = atomics;

        match (bb, be, eb, ee) {
            // bf(s,t):
            // = { EB1(s,t) }
            (_, _, Less, _) => Self::Precedes,
            // bfi(s,t):
            // = { BE−1 }
            // = ¬(BE1(s,t) ∨ BE0(s,t))
            (_, Greater, _, _) => Self::IsPrecededBy,
            // m(s,t):
            // = { EB0(s,t) }
            (_, _, Equal, _) => Self::Meets,
            // mi(s,t):
            // = { BE0(s,t) }
            (_, Equal, _, _) => Self::IsMetBy,
            // f(s,t):
            // = { EE0(s,t) ∧ BB−1(s,t) }
            // = { EE0(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t)) }
            (Greater, _, _, Equal) => Self::Finishes,
            // fi(s,t):
            // = { BB1(s,t) ∧ EE0(s,t) }
            (Less, _, _, Equal) => Self::IsFinishedBy,
            // st(s,t):
            // = { BB0(s,t) ∧ EE1(s,t) }
            (Equal, _, _, Less) => Self::Starts,
            // sti(s,t):
            // = { BB0(s,t) ∧ EE−1(s,t) }
            // = { BB0(s,t) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }
            (Equal, _, _, Greater) => Self::IsStartedBy,
            // di(s,t):
            // = { BB1(s,t) ∧ EE−1(s,t) }
            // = { BB1(s,t) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }
            (Less, _, _, Greater) => Self::Contains,
            // d(s,t):
            // = { EE1(s,t) ∧ BB−1(s,t) }
            // = { EE1(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t)) }
            (Greater, _, _, Less) => Self::IsContainedBy,
            // eq(s,t):
            // = { BB0(s,t) ∧ EE0(s,t) }
            (Equal, _, _, Equal) => Self::Equals,
            // ov(s,t):
            // = { BB1(s,t) ∧ EB−1(s,t) ∧ EE1(s,t) }
            // = { (BB1(s,t) ∧ EE1(s,t)) ∧ ¬(EB0(s,t) ∨ EB1(s,t)) }
            (Less, _, Greater, Less) => Self::Overlaps,
            // ovi(s,t):
            // = { BB−1(s,t) ∧ BE1(s,t) ∧ EE−1(s,t) }
            // = { (BE1(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t))) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }
            (Greater, Less, _, Greater) => Self::IsOverlappedBy,
        }
    }
}

impl Relation {
    /// Returns the allen relation between discrete ranges `s` and `t`
    /// or `None` if any comparisons failures are encountered.
    #[inline]
    pub fn from_discrete_ranges<S, T, U>(s: S, t: T) -> Option<Self>
    where
        S: RangeBounds<U, Discrete>,
        T: RangeBounds<U, Discrete>,
        U: Clone + PartialEq + PartialOrd + Bounded,
    {
        Self::from_ranges(s, t)
    }

    /// Returns the allen relation between non-discrete ranges `s` and `t`
    /// or `None` if any comparisons failures are encountered.
    #[inline]
    pub fn from_non_discrete_ranges<S, T, U>(s: S, t: T) -> Option<Self>
    where
        S: RangeBounds<U, NonDiscrete>,
        T: RangeBounds<U, NonDiscrete>,
        U: Clone + PartialEq + PartialOrd + Bounded,
    {
        Self::from_ranges(s, t)
    }

    /// Returns the allen relation between ranges `s` and `t`
    /// or `None` if any comparisons failures are encountered.
    #[inline]
    pub fn from_ranges<S, T, U, D>(s: S, t: T) -> Option<Self>
    where
        S: RangeBounds<U, D>,
        T: RangeBounds<U, D>,
        U: Clone + PartialEq + PartialOrd + Bounded,
        D: Discreteness,
    {
        AtomicRelations::from_ranges(s, t).map(Relation::from)
    }

    /// Returns the relation's converse.
    pub fn as_converse(&self) -> Self {
        match self {
            Self::Precedes => Self::IsPrecededBy,
            Self::Meets => Self::IsMetBy,
            Self::Overlaps => Self::IsOverlappedBy,
            Self::IsFinishedBy => Self::Finishes,
            Self::Contains => Self::IsContainedBy,
            Self::Starts => Self::IsStartedBy,
            Self::Equals => Self::Equals,
            Self::IsStartedBy => Self::Starts,
            Self::IsContainedBy => Self::Contains,
            Self::Finishes => Self::IsFinishedBy,
            Self::IsOverlappedBy => Self::Overlaps,
            Self::IsMetBy => Self::Meets,
            Self::IsPrecededBy => Self::Precedes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod converses {
        use super::*;

        #[test]
        fn asymmetric() {
            let relations = [
                Relation::Precedes,
                Relation::Meets,
                Relation::Overlaps,
                Relation::IsFinishedBy,
                Relation::Contains,
                Relation::Starts,
                Relation::IsStartedBy,
                Relation::IsContainedBy,
                Relation::Finishes,
                Relation::IsOverlappedBy,
                Relation::IsMetBy,
                Relation::IsPrecededBy,
            ];

            for relation in relations {
                let first_converse = relation.as_converse();
                assert_ne!(relation, first_converse);
                let second_converse = first_converse.as_converse();
                assert_eq!(relation, second_converse);
            }
        }

        #[test]
        fn symmetric() {
            let relations = [Relation::Equals];

            for relation in relations {
                let first_converse = relation.as_converse();
                assert_eq!(relation, first_converse);
                let second_converse = first_converse.as_converse();
                assert_eq!(relation, second_converse);
            }
        }
    }

    mod precedes {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::Precedes);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ────────────────┐
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(..4, 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ────────────────┐
            // t:                          └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(..4, 5..8), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:      ┌──────────────┐
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(1..4, 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:      ┌──────────────┐
            // t:                          └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(1..4, 5..8), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ────────────────┐─ ─ ┐
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(..=3, 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ────────────────┐─ ─ ┐
            // t:                          └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(..=3, 5..=7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:      ┌──────────────┐─ ─ ┐
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(1..=3, 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:      ┌──────────────┐─ ─ ┐
            // t:                          └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(1..=3, 5..=7), EXPECTED);
        }
    }

    mod is_preceded_by {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::IsPrecededBy);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t: ─ ─ ────────────────┘
            assert_eq!(Relation::from_discrete_ranges(5.., ..4), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐
            // t: ─ ─ ────────────────┘
            assert_eq!(Relation::from_discrete_ranges(5..8, ..4), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t:      └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(5.., 1..4), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐
            // t:      └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(5..8, 1..4), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t: ─ ─ ────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(5.., ..=3), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐─ ─ ┐
            // t: ─ ─ ────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(5..=7, ..=3), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t:      └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(5.., 1..=3), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐─ ─ ┐
            // t:      └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(5..=7, 1..=3), EXPECTED);
        }
    }

    mod meets {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::Meets);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ─────────────────────┐
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(..5, 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ─────────────────────┐
            // t:                          └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(..5, 5..8), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:           ┌──────────────┐
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(2..5, 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:           ┌──────────────┐
            // t:                          └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(2..5, 5..8), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ─────────────────────┐─ ─ ┐
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(..=5, 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ─────────────────────┐─ ─ ┐
            // t:                          └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(..=5, 5..=8), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:           ┌──────────────┐─ ─ ┐
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(2..=5, 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:           ┌──────────────┐─ ─ ┐
            // t:                          └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(2..=5, 5..=8), EXPECTED);
        }
    }

    mod is_met_by {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::IsMetBy);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t: ─ ─ ─────────────────────┘
            assert_eq!(Relation::from_discrete_ranges(5.., ..5), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐
            // t: ─ ─ ─────────────────────┘
            assert_eq!(Relation::from_discrete_ranges(5..8, ..5), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t:           └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(5.., 2..5), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐
            // t:           └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(5..8, 2..5), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t: ─ ─ ─────────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(5.., ..=5), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐─ ─ ┐
            // t: ─ ─ ─────────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(5..=8, ..=5), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t:           └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(5.., 2..=5), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐─ ─ ┐
            // t:           └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(5..=8, 2..=5), EXPECTED);
        }
    }

    mod overlaps_with {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::Overlaps);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ──────────────────────────┐
            // t:                     └────────────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(..6, 4..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ──────────────────────────┐
            // t:                     └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(..6, 4..7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌──────────────┐
            // t:                     └────────────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(3..6, 4..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌──────────────┐
            // t:                     └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(3..6, 4..7), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ──────────────────────────┐─ ─ ┐
            // t:                     └────────────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(..=6, 4..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ──────────────────────────┐─ ─ ┐
            // t:                     └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(..=6, 4..=7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌──────────────┐─ ─ ┐
            // t:                     └────────────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(3..=6, 4..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌──────────────┐─ ─ ┐
            // t:                     └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(3..=6, 4..=7), EXPECTED);
        }
    }

    mod is_overlapped_by {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::IsOverlappedBy);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌────────────────────────── ─ ─
            // t: ─ ─ ──────────────────────────┘
            assert_eq!(Relation::from_discrete_ranges(4.., ..6), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐
            // t: ─ ─ ──────────────────────────┘
            assert_eq!(Relation::from_discrete_ranges(4..7, ..6), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌────────────────────────── ─ ─
            // t:                └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(4.., 3..6), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐
            // t:                └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(4..7, 3..6), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌────────────────────────── ─ ─
            // t: ─ ─ ──────────────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4.., ..=6), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐─ ─ ┐
            // t: ─ ─ ──────────────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4..=7, ..=6), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌────────────────────────── ─ ─
            // t:                └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4.., 3..=6), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐─ ─ ┐
            // t:                └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4..=7, 3..=6), EXPECTED);
        }
    }

    mod starts {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::Starts);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐
            // t:                     └────────────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(4..7, 4..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐
            // t:                     └───────────────────┘
            assert_eq!(Relation::from_discrete_ranges(4..7, 4..8), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐─ ─ ┐
            // t:                     └────────────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(4..=7, 4..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐─ ─ ┐
            // t:                     └───────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4..=7, 4..=8), EXPECTED);
        }
    }

    mod is_started_by {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::IsStartedBy);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌────────────────────────── ─ ─
            // t:                     └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(4.., 4..7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌───────────────────┐
            // t:                     └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(4..8, 4..7), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌────────────────────────── ─ ─
            // t:                     └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4.., 4..=7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌───────────────────┐─ ─ ┐
            // t:                     └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4..=8, 4..=7), EXPECTED);
        }
    }

    mod contains {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::Contains);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌─────────────────────────────── ─ ─
            // t:                     └─────────┘
            assert_eq!(Relation::from_discrete_ranges(3.., 4..6), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌───────────────────┐
            // t:                     └─────────┘
            assert_eq!(Relation::from_discrete_ranges(3..7, 4..6), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌─────────────────────────────── ─ ─
            // t:                     └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(3.., 4..=7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌───────────────────┐─ ─ ┐
            // t:                     └─────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(3..=7, 4..=6), EXPECTED);
        }
    }

    mod is_contained_by {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::IsContainedBy);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌─────────┐
            // t:                └─────────────────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(4..6, 3..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌─────────┐
            // t:                └───────────────────┘
            assert_eq!(Relation::from_discrete_ranges(4..6, 3..7), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌─────────┐─ ─ ┐
            // t:                └─────────────────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(4..=7, 3..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌─────────┐─ ─ ┐
            // t:                └───────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4..=6, 3..=7), EXPECTED);
        }
    }

    mod finishes {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::Finishes);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐
            // t: ─ ─ ───────────────────────────────┘
            assert_eq!(Relation::from_discrete_ranges(4..7, ..7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐
            // t:                └───────────────────┘
            assert_eq!(Relation::from_discrete_ranges(4..7, 3..7), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌──────────────┐─ ─ ┐
            // t: ─ ─ ───────────────────────────────┘
            assert_eq!(Relation::from_non_discrete_ranges(4..=7, ..=7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌──────────────┐─ ─ ┐
            // t:                     └───────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4..=7, 3..=7), EXPECTED);
        }
    }

    mod is_finished_by {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::IsFinishedBy);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // t: ─ ─ ───────────────────────────────┐
            // t:                     └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(..7, 4..7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌───────────────────┐
            // t:                     └──────────────┘
            assert_eq!(Relation::from_discrete_ranges(3..7, 4..7), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // t: ─ ─ ───────────────────────────────┐─ ─ ┐
            // t:                     └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(..=7, 4..=7), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                ┌───────────────────┐─ ─ ┐
            // t:                     └──────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(3..=7, 4..=7), EXPECTED);
        }
    }

    mod equals {
        use super::*;

        const EXPECTED: Option<Relation> = Some(Relation::Equals);

        #[test]
        fn discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ─────────────────────────────────────────── ─ ─
            // t: ─ ─ ─────────────────────────────────────────── ─ ─
            assert_eq!(
                Relation::from_discrete_ranges::<_, _, isize>(.., ..),
                EXPECTED
            );

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_discrete_ranges(5.., 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ─────────────────────┐
            // t: ─ ─ ─────────────────────┘
            assert_eq!(Relation::from_discrete_ranges(..5, ..5), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌─────────┐
            // t:                     └─────────┘
            assert_eq!(Relation::from_discrete_ranges(4..6, 4..6), EXPECTED);
        }

        #[test]
        fn non_discrete() {
            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ─────────────────────────────────────────── ─ ─
            // t: ─ ─ ─────────────────────────────────────────── ─ ─
            assert_eq!(
                Relation::from_non_discrete_ranges::<_, _, isize>(.., ..),
                EXPECTED
            );

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                          ┌───────────────────── ─ ─
            // t:                          └───────────────────── ─ ─
            assert_eq!(Relation::from_non_discrete_ranges(5.., 5..), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s: ─ ─ ─────────────────────┐─ ─ ┐
            // t: ─ ─ ─────────────────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(..=5, ..=5), EXPECTED);

            //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
            // s:                     ┌─────────┐─ ─ ┐
            // t:                     └─────────┘─ ─ ┘
            assert_eq!(Relation::from_non_discrete_ranges(4..=6, 4..=6), EXPECTED);
        }
    }
}
