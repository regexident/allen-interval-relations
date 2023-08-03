/// Methods for checking for a "meets" relation between intervals.
pub trait Meets<T>: Sized {
    /// Returns `true` iff `self` meets `other.0.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other:          └────────┘
    /// ```
    #[inline]
    fn meets(&self, _other: &T) -> bool {
        false
    }

    /// Returns `true` iff `self` is met by `other.0.
    ///
    /// ```plain
    /// self:           ┌────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn is_met_by(&self, other: &T) -> bool
    where
        T: Meets<Self>,
    {
        other.meets(self)
    }
}
