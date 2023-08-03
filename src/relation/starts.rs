/// Methods for checking for a "starts" relation between intervals.
pub trait Starts<T>: Sized {
    /// Returns `true` iff `self` starts `other.0.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other: └────────────────┘
    /// ```
    #[inline]
    fn starts(&self, _other: &T) -> bool {
        false
    }

    /// Returns `true` iff `self` is started by `other.0.
    ///
    /// ```plain
    /// self:  ┌────────────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn is_started_by(&self, other: &T) -> bool
    where
        T: Starts<Self>,
    {
        other.starts(self)
    }
}
