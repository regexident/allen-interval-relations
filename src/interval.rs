#[allow(clippy::module_inception)]
mod interval;
mod interval_from;
mod interval_full;
mod interval_to;

pub use self::{interval::*, interval_from::*, interval_full::*, interval_to::*};

/// Error type specific to Allen's interval algebra.
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntervalError {
    /// Empty intervals are invalid with respect to Allen's interval algebra.
    #[error("empty interval, which is not supported by Allen's interval algebra")]
    EmptyInterval,
    /// Could not obtain total order.
    #[error("could not obtain total order")]
    AmbiguousOrder,
}
