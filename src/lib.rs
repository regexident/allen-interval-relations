//! In 1983 James F. Allen published a paper in which he proposed [thirteen basic relations between time intervals][allen-interval-algebra],
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
//! Allen's intervals support both, [discrete][time-domains] and [continuous][time-domains] [time domains][time-domains].
//!
//! ## Discrete (i.e. quantized) time-domain
//!
//! > [Wikipedia][time-domains]: Discrete time views values of variables as occurring at distinct, separate "points in time",
//! > or equivalently as being unchanged throughout each non-zero region of time ("time period")
//! > —that is, time is viewed as a discrete variable.
//! > Thus a non-time variable jumps from one value to another as time moves from one time period to the next.
//! > This view of time corresponds to a digital clock that gives a fixed reading of 10:37 for a while,
//! > and then jumps to a new fixed reading of 10:38, etc.
//! > In this framework, each variable of interest is measured once at each time period.
//! > The number of measurements between any two time periods is finite.
//! > Measurements are typically made at sequential integer values of the variable "time".
//!
//! If your time-values are represented using an integer type (e.g. `i32` or `u32`), then your time domain is most likely discrete.
//!
//! > 💡 In discrete time domains Allen's intervals behave like as if they had exclusive end bounds
//! > (similar to exclusive ranges: `..`, `..y`, `x..` and `x..y`).
//!
//! > ⚠️ Values in discrete (i.e. quantized) domains have a length.
//! > As such an interval `IntervalTo { end: x }` is considered
//! > meeting a range `IntervalFrom { start: x }`, rather than overlapping it.
//!
//! ## Continuous (i.e. un-quantized) time-domain
//!
//! > [Wikipedia][time-domains]: Continuous time views variables as having a particular value only for an infinitesimally short amount of time.
//! > Between any two points in time there are an infinite number of other points in time.
//! > The variable "time" ranges over the entire real number line, or depending on the context,
//! > over some subset of it such as the non-negative reals. Thus time is viewed as a continuous variable.
//! >
//! > A continuous signal or a continuous-time signal is a varying quantity (a signal) whose domain,
//! > which is often time, is a continuum (e.g., a connected interval of the reals).
//! > That is, the function's domain is an uncountable set. The function itself need not to be continuous.
//! > To contrast, a discrete-time signal has a countable domain, like the natural numbers.
//!
//! If your time-values are represented using a floating-point type (e.g. `f32` or `f64`), then your time domain is most likely continuous.
//!
//! > 💡 In continuous time domains Allen's intervals behave like as if they had exclusive end bounds
//! > (similar to inclusive ranges:`..`, `..=y`, `x..` and `x..=y`).
//!
//! > ⚠️ Values in continuous domains have an infinitesimally short (i.e. ~0.0) length.
//! > As such a range `IntervalTo { end: x.y }`` is considered
//! > meeting a range `IntervalFrom { start: x }`, rather than overlapping it.
//!
//! # Examples
//!
//! ```
//! use allen_intervals::{Contains, FromIntervals, Interval, Meets, NonEmpty, Precedes, Relation};
//!
//! // Allen's interval algebra is only defined for non-empty intervals.
//! // We thus need to wrap them in `NonEmpty<T>` first:
//! let s: NonEmpty<_> = Interval { start: 1, end: 4 }.try_into().unwrap();
//! let t: NonEmpty<_> = Interval { start: 5, end: 8 }.try_into().unwrap();
//!
//! assert_eq!(
//!     Relation::from_intervals(&s, &t),
//!     Relation::Precedes { is_inverted: false }
//! );
//! assert_eq!(
//!     Relation::from_intervals(&t, &s),
//!     Relation::Precedes { is_inverted: true }
//! );
//!
//! assert!(s.precedes(&t));
//! assert!(t.is_preceded_by(&s));
//!
//! // Allen's interval algebra is only defined for non-empty intervals.
//! // We thus need to wrap them in `NonEmpty<T>` first:
//! let s: NonEmpty<_> = Interval { start: 1, end: 5 }.try_into().unwrap();
//! let t: NonEmpty<_> = Interval { start: 5, end: 9 }.try_into().unwrap();
//!
//! assert_eq!(
//!     Relation::from_intervals(&s, &t),
//!     Relation::Meets { is_inverted: false }
//! );
//! assert_eq!(
//!     Relation::from_intervals(&t, &s),
//!     Relation::Meets { is_inverted: true }
//! );
//!
//! assert!(s.meets(&t));
//! assert!(t.is_met_by(&s));
//!
//! // Allen's interval algebra is only defined for non-empty intervals.
//! // We thus need to wrap them in `NonEmpty<T>` first:
//! let s: NonEmpty<_> = Interval { start: 3, end: 7 }.try_into().unwrap();
//! let t: NonEmpty<_> = Interval { start: 4, end: 6 }.try_into().unwrap();
//!
//! assert_eq!(
//!     Relation::from_intervals(&s, &t),
//!     Relation::Contains { is_inverted: false }
//! );
//! assert_eq!(
//!     Relation::from_intervals(&t, &s),
//!     Relation::Contains { is_inverted: true }
//! );
//!
//! assert!(s.contains(&t));
//! assert!(t.is_contained_by(&s));
//! ```
//!
//! [allen-interval-algebra]: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
//! [quantization]: https://en.wikipedia.org/wiki/Quantization
//! [time-domains]: https://en.wikipedia.org/wiki/Discrete_time_and_continuous_time

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![warn(missing_docs)]

#[cfg(all(not(test), not(feature = "std")))]
extern crate core as std;

#[cfg(all(test, feature = "std"))]
extern crate std;

mod atomic;
mod bounds;
mod from_intervals;
mod interval;
mod non_empty;
mod relation;

pub use self::{bounds::*, from_intervals::*, interval::*, non_empty::*, relation::*};

use self::atomic::*;
