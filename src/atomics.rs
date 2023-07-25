use std::cmp::Ordering;

use crate::{Bounded, Discrete, Discreteness, NonDiscrete, RangeBounds};

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
    /// Returns the atomic allen relations between discrete ranges `s` and `t`
    /// or `None` if any comparisons failures are encountered.
    #[inline]
    pub fn from_discrete_ranges<S, T, U>(s: S, t: T) -> Option<Self>
    where
        S: RangeBounds<U, Discrete>,
        T: RangeBounds<U, Discrete>,
        U: PartialOrd + Bounded,
    {
        Self::from_ranges::<S, T, U, Discrete>(s, t)
    }

    /// Returns the atomic allen relations between non-discrete ranges `s` and `t`
    /// or `None` if any comparisons failures are encountered.
    #[inline]
    pub fn from_non_discrete_ranges<S, T, U>(s: S, t: T) -> Option<Self>
    where
        S: RangeBounds<U, NonDiscrete>,
        T: RangeBounds<U, NonDiscrete>,
        U: PartialOrd + Bounded,
    {
        Self::from_ranges::<S, T, U, NonDiscrete>(s, t)
    }

    /// Returns the atomic allen relations between ranges `s` and `t`
    /// or `None` if any comparisons failures are encountered.
    #[inline]
    pub fn from_ranges<S, T, U, D>(s: S, t: T) -> Option<Self>
    where
        S: RangeBounds<U, D>,
        T: RangeBounds<U, D>,
        U: PartialOrd + Bounded,
        D: Discreteness,
    {
        let sb = s.start_bound();
        let se = s.end_bound();
        let tb = t.start_bound();
        let te = t.end_bound();

        assert!(sb <= se, "empty ranges are not supported");
        assert!(tb <= te, "empty ranges are not supported");

        let bb = sb.partial_cmp(&tb)?;
        let be = sb.partial_cmp(&te)?;
        let eb = se.partial_cmp(&tb)?;
        let ee = se.partial_cmp(&te)?;

        Some(Self { bb, be, eb, ee })
    }

    /// Returns the ordering between `s`'s start bound and `t`'s start bound.
    pub fn bb(&self) -> Ordering {
        self.bb
    }

    /// Returns the ordering between `s`'s start bound and `t`'s end bound.
    pub fn be(&self) -> Ordering {
        self.be
    }

    /// Returns the ordering between `s`'s end bound and `t`'s start bound.
    pub fn eb(&self) -> Ordering {
        self.eb
    }

    /// Returns the ordering between `s`'s end bound and `t`'s end bound.
    pub fn ee(&self) -> Ordering {
        self.ee
    }
}
