use crate::sealed::Sealed;

/// A phantom type to denote whether range bounds represent discrete or non-discrete times.
pub trait Discreteness: Sealed {}

/// A phantom type for non-discrete time ranges (e.g. `RangeInclusive<f32>`).
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum NonDiscrete {}

impl Sealed for NonDiscrete {}
impl Discreteness for NonDiscrete {}

/// A phantom type for discrete time ranges (e.g. `Range<i32>`).
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Discrete {}

impl Sealed for Discrete {}
impl Discreteness for Discrete {}
