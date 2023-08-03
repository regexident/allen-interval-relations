/// Methods for checking for a "equals" relation between intervals.
pub trait Equals<T>: Sized {
    /// Returns `true` iff `self` equals `other.0.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn equals(&self, _other: &T) -> bool {
        false
    }
}
