#[allow(clippy::module_inception)]
mod interval;
mod interval_from;
mod interval_full;
mod interval_to;

pub use self::{interval::*, interval_from::*, interval_full::*, interval_to::*};

/// Error type specific to Allen's interval algebra.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntervalError {
    /// Empty intervals are invalid with respect to Allen's interval algebra.
    EmptyInterval,
    /// Could not obtain total order.
    AmbiguousOrder,
}
