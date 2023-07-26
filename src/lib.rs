//! In 1983 James F. Allen published a paper in which he proposed [thirteen basic relations between time intervals][allen-interval-algebra]
//! that are distinct, exhaustive, and qualitative:
//!
//! > Allen, J. F. (1983).
//! > Maintaining knowledge about temporal intervals.
//! > Communications of the ACM, 26(11), 832-843.
//!
//! - *distinct*, because no pair of definite intervals can be related by more than one of the relationships
//! - *exhaustive*, because any pair of definite intervals are described by one of the relations
//! - *qualitative*, (rather than quantitative) because no numeric time spans are considered
//!
//! # Examples
//!
//! This crate implements Allen's intervals for both, discrete (i.e. [quantized][quantization]) and non-discrete (aka. continuous) (i.e. [un-quantized][quantization]) time domains.
//!
//! ## Non-discrete (i.e. un-quantized) time-domain
//!
//! If your time-values are represented using a floating-point type (e.g. `f32` or `f64`), then your time domain is most likely non-discrete.
//!
//! > 💡 Non-discrete time domains support the use of `..`, `..=y`, `x..` and `x..=y` (i.e. inclusive) ranges.
//!
//! > ⚠️ Values in non-discrete (i.e. continuous) domains have no width. As such a range `..=x` is considered meeting a range `x..`, rather than overlapping it.
//!
//! ```
//! use allen_interval_relations::{FromRanges, Relation};
//!
//! assert_eq!(Relation::from_ranges(2.0..=4.0, 5.0..=8.0), Some(Relation::Precedes { is_inverted: false }));
//! assert_eq!(Relation::from_ranges(2.0..=5.0, 5.0..=8.0), Some(Relation::Meets { is_inverted: false }));
//! assert_eq!(Relation::from_ranges(2.0..=6.0, 5.0..=8.0), Some(Relation::Overlaps { is_inverted: false }));
//! ```
//!
//! ## Discrete (i.e. quantized) time-domain
//!
//! If your time-values however are represented using an integer type (e.g. `f32` or `f64`), then your time domain is most likely non-discrete.
//!
//! > 💡 Discrete time domains support the use of `..`, `..y`, `x..` and `x..y` (i.e. exclusive) ranges.
//!
//! > ⚠️ Values in discrete (i.e. quantized) domains have a width. As such a range `..x` is considered overlapping a range `x..`, rather than meeting it.
//!
//! ```
//! use allen_interval_relations::{FromRanges, Relation};
//!
//! assert_eq!(Relation::from_ranges(2..4, 5..8), Some(Relation::Precedes { is_inverted: false }));
//! assert_eq!(Relation::from_ranges(2..5, 5..8), Some(Relation::Meets { is_inverted: false }));
//! assert_eq!(Relation::from_ranges(2..6, 5..8), Some(Relation::Overlaps { is_inverted: false }));
//! ```
//!
//! [allen-interval-algebra]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
//! [quantization]: https://en.wikipedia.org/wiki/Quantization

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![warn(missing_docs)]

#[cfg(all(not(test), not(feature = "std")))]
extern crate core as std;

#[cfg(all(test, feature = "std"))]
extern crate std;

mod atomic_relations;
mod bounded;
mod from_ranges;
mod relation;

pub use self::{atomic_relations::*, bounded::*, from_ranges::*, relation::*};
