use std::cmp::Ordering;

use crate::{
    Bb, Be, Bounds, Eb, Ee, FromIntervals, Interval, IntervalBounds, IntervalError, IntervalFrom,
    IntervalFull, IntervalTo, NonEmpty, TryFromIntervals,
};

mod contains;
mod equals;
mod finishes;
mod meets;
mod overlaps;
mod precedes;
mod starts;

pub use self::{
    contains::*, equals::*, finishes::*, meets::*, overlaps::*, precedes::*, starts::*,
};

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
    /// s:         ┌────────┐
    /// t: └────────────────┘
    ///
    /// `Finishes { is_inverted: true }`:
    ///
    /// ```plain
    /// s: ┌────────────────┐
    /// t:         └────────┘
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

impl Relation {
    #[inline]
    fn from_bounds<T>(s: &Bounds<T>, t: &Bounds<T>) -> Self
    where
        T: Ord,
    {
        let bb = Bb::from_bounds(&s.start, &t.start);
        let be = Be::from_bounds(&s.start, &t.end);
        let eb = Eb::from_bounds(&s.end, &t.start);
        let ee = Ee::from_bounds(&s.end, &t.end);

        Self::from_atomic_relations(bb, be, eb, ee)
    }

    #[inline]
    fn try_from_bounds<T>(s: &Bounds<T>, t: &Bounds<T>) -> Result<Self, IntervalError>
    where
        T: PartialOrd,
    {
        let bb = Bb::try_from_bounds(&s.start, &t.start)?;
        let be = Be::try_from_bounds(&s.start, &t.end)?;
        let eb = Eb::try_from_bounds(&s.end, &t.start)?;
        let ee = Ee::try_from_bounds(&s.end, &t.end)?;

        Ok(Self::from_atomic_relations(bb, be, eb, ee))
    }

    /// Each of Allen’s relations can be reduced to a boolean combination of
    /// a combination of atomic relations.
    /// By computing each of the atomic relations only once and only if needed,
    /// we can decrease the overall runtime of the computation of Allen relations.
    ///
    /// See the following paper for more info:
    ///
    /// > Georgala, K., Sherif, M. A., & Ngonga Ngomo, A. C. (2016).
    /// > An efficient approach for the generation of Allen relations.
    /// > In ECAI 2016 (pp. 948-956). IOS Press.
    #[inline]
    fn from_atomic_relations(bb: Bb, be: Be, eb: Eb, ee: Ee) -> Self {
        use Ordering::*;

        match (bb.0, be.0, eb.0, ee.0) {
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

impl Ord for Relation {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.order().cmp(&other.order())
    }
}

impl PartialOrd for Relation {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromIntervals<IntervalFull, IntervalFull> for Relation {
    #[inline]
    fn from_intervals(s: &NonEmpty<IntervalFull>, t: &NonEmpty<IntervalFull>) -> Self {
        assert_eq!(s, t);

        let bb = Bb(Ordering::Equal);
        let be = Be(Ordering::Less);
        let eb = Eb(Ordering::Greater);
        let ee = Ee(Ordering::Equal);

        Self::from_atomic_relations(bb, be, eb, ee)
    }
}

impl TryFromIntervals<IntervalFull, IntervalFull> for Relation {
    #[inline]
    fn try_from_intervals(
        s: &NonEmpty<IntervalFull>,
        t: &NonEmpty<IntervalFull>,
    ) -> Result<Self, IntervalError> {
        assert_eq!(s, t);

        let bb = Bb(Ordering::Equal);
        let be = Be(Ordering::Less);
        let eb = Eb(Ordering::Greater);
        let ee = Ee(Ordering::Equal);

        Ok(Self::from_atomic_relations(bb, be, eb, ee))
    }
}

macro_rules! from_intervals_impl {
    ($s:ty, $t:ty) => {
        impl<T> FromIntervals<$s, $t> for Relation
        where
            T: Ord + Copy,
        {
            fn from_intervals(s: &NonEmpty<$s>, t: &NonEmpty<$t>) -> Self {
                Self::from_bounds(&s.0.bounds(), &t.0.bounds())
            }
        }

        impl<T> TryFromIntervals<$s, $t> for Relation
        where
            T: PartialOrd + Copy,
        {
            fn try_from_intervals(
                s: &NonEmpty<$s>,
                t: &NonEmpty<$t>,
            ) -> Result<Self, IntervalError> {
                Self::try_from_bounds(&s.0.bounds(), &t.0.bounds())
            }
        }
    };
}

from_intervals_impl!(IntervalFull, IntervalTo<T>);
from_intervals_impl!(IntervalFull, IntervalFrom<T>);
from_intervals_impl!(IntervalFull, Interval<T>);

from_intervals_impl!(IntervalTo<T>, IntervalFull);
from_intervals_impl!(IntervalTo<T>, IntervalTo<T>);
from_intervals_impl!(IntervalTo<T>, IntervalFrom<T>);
from_intervals_impl!(IntervalTo<T>, Interval<T>);

from_intervals_impl!(IntervalFrom<T>, IntervalFull);
from_intervals_impl!(IntervalFrom<T>, IntervalTo<T>);
from_intervals_impl!(IntervalFrom<T>, IntervalFrom<T>);
from_intervals_impl!(IntervalFrom<T>, Interval<T>);

from_intervals_impl!(Interval<T>, IntervalFull);
from_intervals_impl!(Interval<T>, IntervalTo<T>);
from_intervals_impl!(Interval<T>, IntervalFrom<T>);
from_intervals_impl!(Interval<T>, Interval<T>);

#[cfg(test)]
mod tests;
