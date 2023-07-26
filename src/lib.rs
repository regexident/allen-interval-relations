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
//! Non-discrete time domains support the use of `..`, `..=y`, `x..` and `x..=y` (i.e. inclusive) ranges.
//!
//! ```
//! use allen_intervals::{FromRanges, Relation};
//!
//! assert_eq!(Relation::from_ranges(2.0..=5.0, 5.0..=8.0), Some(Relation::Meets));
//! ```
//!
//! ## Discrete (i.e. quantized) time-domain
//!
//! If your time-values however are represented using an integer type (e.g. `f32` or `f64`), then your time domain is most likely non-discrete.
//! Non-discrete time domains support the use of `..`, `..y`, `x..` and `x..y` (i.e. exclusive) ranges.
//!
//! ```
//! use allen_intervals::{FromRanges, Relation};
//!
//! assert_eq!(Relation::from_ranges(2..5, 5..8), Some(Relation::Meets));
//! ```
//!
//! [allen-interval-algebra]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
//! [quantization]: https://en.wikipedia.org/wiki/Quantization

#![warn(missing_docs)]

mod atomics;
mod bounded;
mod from_ranges;
mod relation;

pub use self::{atomics::*, bounded::*, from_ranges::*, relation::*};
