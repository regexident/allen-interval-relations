use std::cmp::Ordering;

use crate::{AtomicRelations, FromRanges};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum RelationOrder {
    Precedes,
    Meets,
    Overlaps,
    IsFinishedBy,
    Contains,
    Starts,
    Equals,
    IsStartedBy,
    IsContainedBy,
    Finishes,
    IsOverlappedBy,
    IsMetBy,
    IsPrecededBy,
}

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
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Relation {
    /// `Precedes { is_inverted: false }`:
    ///
    /// ```plain
    /// s: ┌────────┐
    /// t:            └────────┘
    /// ```
    ///
    /// `Precedes { is_inverted: true }`:
    ///
    /// ```plain
    /// s:            ┌────────┐
    /// t: └────────┘
    /// ```
    Precedes {
        /// `true` iff the relation is inverted, otherwise `false.
        ///
        /// - `false` => "s precedes t"
        /// - `true` => "s is preceded by t"
        is_inverted: bool,
    },
    /// `Meets { is_inverted: false }`:
    ///
    /// ```plain
    /// s: ┌────────┐
    /// t:          └────────┘
    /// ```
    ///
    /// `Meets { is_inverted: true }`:
    ///
    /// ```plain
    /// s:          ┌────────┐
    /// t: └────────┘
    /// ```
    Meets {
        /// `true` iff the relation is inverted, otherwise `false.
        ///
        /// - `false` => "s meets t"
        /// - `true` => "s is met by t"
        is_inverted: bool,
    },
    /// `Overlaps { is_inverted: false }`:
    ///
    /// ```plain
    /// s: ┌────────┐
    /// t:      └────────┘
    /// ```
    ///
    /// `Overlaps { is_inverted: true }`:
    ///
    /// ```plain
    /// s:      ┌────────┐
    /// t: └────────┘
    /// ```
    Overlaps {
        /// `true` iff the relation is inverted, otherwise `false.
        ///
        /// - `false` => "s overlaps t"
        /// - `true` => "s is overlapped by t"
        is_inverted: bool,
    },
    /// `Finishes { is_inverted: false }`:
    ///
    /// ```plain
    /// s: ┌────────────────┐
    /// t:         └────────┘
    /// ```
    ///
    /// `Finishes { is_inverted: true }`:
    ///
    /// ```plain
    /// s:         ┌────────┐
    /// t: └────────────────┘
    /// ```
    Finishes {
        /// `true` iff the relation is inverted, otherwise `false.
        ///
        /// - `false` => "s finishes t"
        /// - `true` => "s is finished by t"
        is_inverted: bool,
    },
    /// `Contains { is_inverted: false }`:
    ///
    /// ```plain
    /// s: ┌────────────────┐
    /// t:     └────────┘
    /// ```
    ///
    /// `Contains { is_inverted: true }`:
    ///
    /// ```plain
    /// s:     ┌────────┐
    /// t: └────────────────┘
    /// ```
    Contains {
        /// `true` iff the relation is inverted, otherwise `false.
        ///
        /// - `false` => "s contains t"
        /// - `true` => "s is contained by t"
        is_inverted: bool,
    },
    /// `Starts { is_inverted: false }`:
    ///
    /// ```plain
    /// s: ┌────────┐
    /// t: └────────────────┘
    /// ```
    ///
    /// `Starts { is_inverted: true }`:
    ///
    /// ```plain
    /// s: ┌────────────────┐
    /// t: └────────┘
    /// ```
    Starts {
        /// `true` iff the relation is inverted, otherwise `false.
        ///
        /// - `false` => "s starts t"
        /// - `true` => "s is started by t"
        is_inverted: bool,
    },
    /// `Equals`:
    ///
    /// ```plain
    /// s: ┌────────┐
    /// t: └────────┘
    /// ```
    Equals,
}

impl Ord for Relation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order().cmp(&other.order())
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
            (_, _, Less, _) => Self::Precedes { is_inverted: false },
            // bfi(s,t):
            // = { BE−1 }
            // = ¬(BE1(s,t) ∨ BE0(s,t))
            (_, Greater, _, _) => Self::Precedes { is_inverted: true },
            // m(s,t):
            // = { EB0(s,t) }
            (_, _, Equal, _) => Self::Meets { is_inverted: false },
            // mi(s,t):
            // = { BE0(s,t) }
            (_, Equal, _, _) => Self::Meets { is_inverted: true },
            // f(s,t):
            // = { EE0(s,t) ∧ BB−1(s,t) }
            // = { EE0(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t)) }
            (Greater, _, _, Equal) => Self::Finishes { is_inverted: false },
            // fi(s,t):
            // = { BB1(s,t) ∧ EE0(s,t) }
            (Less, _, _, Equal) => Self::Finishes { is_inverted: true },
            // st(s,t):
            // = { BB0(s,t) ∧ EE1(s,t) }
            (Equal, _, _, Less) => Self::Starts { is_inverted: false },
            // sti(s,t):
            // = { BB0(s,t) ∧ EE−1(s,t) }
            // = { BB0(s,t) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }
            (Equal, _, _, Greater) => Self::Starts { is_inverted: true },
            // di(s,t):
            // = { BB1(s,t) ∧ EE−1(s,t) }
            // = { BB1(s,t) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }
            (Less, _, _, Greater) => Self::Contains { is_inverted: false },
            // d(s,t):
            // = { EE1(s,t) ∧ BB−1(s,t) }
            // = { EE1(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t)) }
            (Greater, _, _, Less) => Self::Contains { is_inverted: true },
            // eq(s,t):
            // = { BB0(s,t) ∧ EE0(s,t) }
            (Equal, _, _, Equal) => Self::Equals,
            // ov(s,t):
            // = { BB1(s,t) ∧ EB−1(s,t) ∧ EE1(s,t) }
            // = { (BB1(s,t) ∧ EE1(s,t)) ∧ ¬(EB0(s,t) ∨ EB1(s,t)) }
            (Less, _, Greater, Less) => Self::Overlaps { is_inverted: false },
            // ovi(s,t):
            // = { BB−1(s,t) ∧ BE1(s,t) ∧ EE−1(s,t) }
            // = { (BE1(s,t) ∧ ¬(BB0(s,t) ∨ BB1(s,t))) ∧ ¬(EE0(s,t) ∨ EE1(s,t)) }
            (Greater, Less, _, Greater) => Self::Overlaps { is_inverted: true },
        }
    }
}

impl<S, T> FromRanges<S, T> for Relation
where
    AtomicRelations: FromRanges<S, T>,
{
    fn from_ranges(s: S, t: T) -> Option<Self> {
        AtomicRelations::from_ranges(s, t).map(Relation::from)
    }
}

impl Relation {
    fn order(&self) -> RelationOrder {
        match self {
            Relation::Precedes { is_inverted: false } => RelationOrder::Precedes,
            Relation::Precedes { is_inverted: true } => RelationOrder::IsPrecededBy,
            Relation::Meets { is_inverted: false } => RelationOrder::Meets,
            Relation::Meets { is_inverted: true } => RelationOrder::IsMetBy,
            Relation::Overlaps { is_inverted: false } => RelationOrder::Overlaps,
            Relation::Overlaps { is_inverted: true } => RelationOrder::IsOverlappedBy,
            Relation::Finishes { is_inverted: false } => RelationOrder::Finishes,
            Relation::Finishes { is_inverted: true } => RelationOrder::IsFinishedBy,
            Relation::Contains { is_inverted: false } => RelationOrder::Contains,
            Relation::Contains { is_inverted: true } => RelationOrder::IsContainedBy,
            Relation::Starts { is_inverted: false } => RelationOrder::IsStartedBy,
            Relation::Starts { is_inverted: true } => RelationOrder::Starts,
            Relation::Equals => RelationOrder::Equals,
        }
    }

    /// Returns the relation's converse.
    pub fn as_converse(&self) -> Self {
        match self {
            Self::Precedes { is_inverted } => Self::Precedes {
                is_inverted: !is_inverted,
            },
            Self::Meets { is_inverted } => Self::Meets {
                is_inverted: !is_inverted,
            },
            Self::Overlaps { is_inverted } => Self::Overlaps {
                is_inverted: !is_inverted,
            },
            Self::Finishes { is_inverted } => Self::Finishes {
                is_inverted: !is_inverted,
            },
            Self::Contains { is_inverted } => Self::Contains {
                is_inverted: !is_inverted,
            },
            Self::Starts { is_inverted } => Self::Starts {
                is_inverted: !is_inverted,
            },
            Self::Equals => Self::Equals,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converses() {
        // Symmetric relations:

        let symmetric_relations = [Relation::Equals];

        for relation in symmetric_relations {
            let first_converse = relation.as_converse();
            assert_eq!(relation, first_converse);

            let second_converse = first_converse.as_converse();
            assert_eq!(relation, second_converse);
        }

        // Asymmetric relations:

        let asymmetric_relations = [
            Relation::Precedes { is_inverted: false },
            Relation::Meets { is_inverted: false },
            Relation::Overlaps { is_inverted: false },
            Relation::Finishes { is_inverted: false },
            Relation::Contains { is_inverted: false },
            Relation::Starts { is_inverted: false },
        ];

        for relation in asymmetric_relations {
            let first_converse = relation.as_converse();
            assert_ne!(relation, first_converse);

            let second_converse = first_converse.as_converse();
            assert_eq!(relation, second_converse);
        }
    }

    #[test]
    fn precedes() {
        const EXPECTED: Option<Relation> = Some(Relation::Precedes { is_inverted: false });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ────────────────┐
        // t:                          └───────────────────── ─ ─
        assert_eq!(Relation::from_ranges(..4, 5..), EXPECTED);
        assert_eq!(Relation::from_ranges(..=4, 5..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ────────────────┐
        // t:                          └──────────────┘
        assert_eq!(Relation::from_ranges(..4, 5..8), EXPECTED);
        assert_eq!(Relation::from_ranges(..=4, 5..=8), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:      ┌──────────────┐
        // t:                          └───────────────────── ─ ─
        assert_eq!(Relation::from_ranges(1..4, 5..), EXPECTED);
        assert_eq!(Relation::from_ranges(1..=4, 5..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:      ┌──────────────┐
        // t:                          └──────────────┘
        assert_eq!(Relation::from_ranges(1..4, 5..8), EXPECTED);
        assert_eq!(Relation::from_ranges(1..=4, 5..=8), EXPECTED);
    }

    #[test]
    fn is_preceded_by() {
        const EXPECTED: Option<Relation> = Some(Relation::Precedes { is_inverted: true });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌───────────────────── ─ ─
        // t: ─ ─ ────────────────┘
        assert_eq!(Relation::from_ranges(5.., ..4), EXPECTED);
        assert_eq!(Relation::from_ranges(5.., ..=4), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌──────────────┐
        // t: ─ ─ ────────────────┘
        assert_eq!(Relation::from_ranges(5..8, ..4), EXPECTED);
        assert_eq!(Relation::from_ranges(5..=8, ..=4), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌───────────────────── ─ ─
        // t:      └──────────────┘
        assert_eq!(Relation::from_ranges(5.., 1..4), EXPECTED);
        assert_eq!(Relation::from_ranges(5.., 1..=4), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌──────────────┐
        // t:      └──────────────┘
        assert_eq!(Relation::from_ranges(5.., 1..4), EXPECTED);
        assert_eq!(Relation::from_ranges(5.., 1..=4), EXPECTED);
    }

    #[test]
    fn meets() {
        const EXPECTED: Option<Relation> = Some(Relation::Meets { is_inverted: false });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ─────────────────────┐
        // t:                          └───────────────────── ─ ─
        assert_eq!(Relation::from_ranges(..5, 5..), EXPECTED);
        assert_eq!(Relation::from_ranges(..5, 5..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ─────────────────────┐
        // t:                          └──────────────┘
        assert_eq!(Relation::from_ranges(..5, 5..8), EXPECTED);
        assert_eq!(Relation::from_ranges(..5, 5..8), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:           ┌──────────────┐
        // t:                          └───────────────────── ─ ─
        assert_eq!(Relation::from_ranges(2..5, 5..), EXPECTED);
        assert_eq!(Relation::from_ranges(2..5, 5..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:           ┌──────────────┐
        // t:                          └──────────────┘
        assert_eq!(Relation::from_ranges(2..5, 5..8), EXPECTED);
        assert_eq!(Relation::from_ranges(2..5, 5..8), EXPECTED);
    }

    #[test]
    fn is_met_by() {
        const EXPECTED: Option<Relation> = Some(Relation::Meets { is_inverted: true });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌───────────────────── ─ ─
        // t: ─ ─ ─────────────────────┘
        assert_eq!(Relation::from_ranges(5.., ..5), EXPECTED);
        assert_eq!(Relation::from_ranges(5.., ..=5), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌──────────────┐
        // t: ─ ─ ─────────────────────┘
        assert_eq!(Relation::from_ranges(5..8, ..5), EXPECTED);
        assert_eq!(Relation::from_ranges(5..=8, ..=5), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌───────────────────── ─ ─
        // t:           └──────────────┘
        assert_eq!(Relation::from_ranges(5.., 2..5), EXPECTED);
        assert_eq!(Relation::from_ranges(5.., 2..=5), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌──────────────┐
        // t:           └──────────────┘
        assert_eq!(Relation::from_ranges(5..8, 2..5), EXPECTED);
        assert_eq!(Relation::from_ranges(5..=8, 2..=5), EXPECTED);
    }

    #[test]
    fn overlaps_with() {
        const EXPECTED: Option<Relation> = Some(Relation::Overlaps { is_inverted: false });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ──────────────────────────┐
        // t:                     └────────────────────────── ─ ─
        assert_eq!(Relation::from_ranges(..6, 4..), EXPECTED);
        assert_eq!(Relation::from_ranges(..=6, 4..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ──────────────────────────┐
        // t:                     └──────────────┘
        assert_eq!(Relation::from_ranges(..6, 4..7), EXPECTED);
        assert_eq!(Relation::from_ranges(..=6, 4..=7), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌──────────────┐
        // t:                     └────────────────────────── ─ ─
        assert_eq!(Relation::from_ranges(3..6, 4..), EXPECTED);
        assert_eq!(Relation::from_ranges(3..=6, 4..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌──────────────┐
        // t:                     └──────────────┘
        assert_eq!(Relation::from_ranges(3..6, 4..7), EXPECTED);
        assert_eq!(Relation::from_ranges(3..=6, 4..=7), EXPECTED);
    }

    #[test]
    fn is_overlapped_by() {
        const EXPECTED: Option<Relation> = Some(Relation::Overlaps { is_inverted: true });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌────────────────────────── ─ ─
        // t: ─ ─ ──────────────────────────┘
        assert_eq!(Relation::from_ranges(4.., ..6), EXPECTED);
        assert_eq!(Relation::from_ranges(4.., ..=6), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t: ─ ─ ──────────────────────────┘
        assert_eq!(Relation::from_ranges(4..7, ..6), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=7, ..=6), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌────────────────────────── ─ ─
        // t:                └──────────────┘
        assert_eq!(Relation::from_ranges(4.., 3..6), EXPECTED);
        assert_eq!(Relation::from_ranges(4.., 3..=6), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t:                └──────────────┘
        assert_eq!(Relation::from_ranges(4..7, 3..6), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=7, 3..=6), EXPECTED);
    }

    #[test]
    fn starts() {
        const EXPECTED: Option<Relation> = Some(Relation::Starts { is_inverted: false });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t:                     └────────────────────────── ─ ─
        assert_eq!(Relation::from_ranges(4..7, 4..), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=7, 4..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t:                     └───────────────────┘
        assert_eq!(Relation::from_ranges(4..7, 4..8), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=7, 4..=8), EXPECTED);
    }

    #[test]
    fn is_started_by() {
        const EXPECTED: Option<Relation> = Some(Relation::Starts { is_inverted: true });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌────────────────────────── ─ ─
        // t:                     └──────────────┘
        assert_eq!(Relation::from_ranges(4.., 4..7), EXPECTED);
        assert_eq!(Relation::from_ranges(4.., 4..=7), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌───────────────────┐
        // t:                     └──────────────┘
        assert_eq!(Relation::from_ranges(4..8, 4..7), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=8, 4..=7), EXPECTED);
    }

    #[test]
    fn contains() {
        const EXPECTED: Option<Relation> = Some(Relation::Contains { is_inverted: false });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌─────────────────────────────── ─ ─
        // t:                     └─────────┘
        assert_eq!(Relation::from_ranges(3.., 4..6), EXPECTED);
        assert_eq!(Relation::from_ranges(3.., 4..=6), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌───────────────────┐
        // t:                     └─────────┘
        assert_eq!(Relation::from_ranges(3..7, 4..6), EXPECTED);
        assert_eq!(Relation::from_ranges(3..=7, 4..=6), EXPECTED);
    }

    #[test]
    fn is_contained_by() {
        const EXPECTED: Option<Relation> = Some(Relation::Contains { is_inverted: true });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌─────────┐
        // t:                └─────────────────────────────── ─ ─
        assert_eq!(Relation::from_ranges(4..6, 3..), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=6, 3..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌─────────┐
        // t:                └───────────────────┘
        assert_eq!(Relation::from_ranges(4..6, 3..7), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=6, 3..=7), EXPECTED);
    }

    #[test]
    fn finishes() {
        const EXPECTED: Option<Relation> = Some(Relation::Finishes { is_inverted: false });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t: ─ ─ ───────────────────────────────┘
        assert_eq!(Relation::from_ranges(4..7, ..7), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=7, ..=7), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌──────────────┐
        // t:                └───────────────────┘
        assert_eq!(Relation::from_ranges(4..7, 3..7), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=7, 3..=7), EXPECTED);
    }

    #[test]
    fn is_finished_by() {
        const EXPECTED: Option<Relation> = Some(Relation::Finishes { is_inverted: true });

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // t: ─ ─ ───────────────────────────────┐
        // t:                     └──────────────┘
        assert_eq!(Relation::from_ranges(..7, 4..7), EXPECTED);
        assert_eq!(Relation::from_ranges(..=7, 4..=7), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                ┌───────────────────┐
        // t:                     └──────────────┘
        assert_eq!(Relation::from_ranges(3..7, 4..7), EXPECTED);
        assert_eq!(Relation::from_ranges(3..=7, 4..=7), EXPECTED);
    }

    #[test]
    fn equals() {
        const EXPECTED: Option<Relation> = Some(Relation::Equals);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ─────────────────────────────────────────── ─ ─
        // t: ─ ─ ─────────────────────────────────────────── ─ ─
        assert_eq!(Relation::from_ranges(.., ..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                          ┌───────────────────── ─ ─
        // t:                          └───────────────────── ─ ─
        assert_eq!(Relation::from_ranges(5.., 5..), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s: ─ ─ ─────────────────────┐
        // t: ─ ─ ─────────────────────┘
        assert_eq!(Relation::from_ranges(..5, ..5), EXPECTED);
        assert_eq!(Relation::from_ranges(..=5, ..=5), EXPECTED);

        //    | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 |
        // s:                     ┌─────────┐
        // t:                     └─────────┘
        assert_eq!(Relation::from_ranges(4..6, 4..6), EXPECTED);
        assert_eq!(Relation::from_ranges(4..=6, 4..=6), EXPECTED);
    }
}
