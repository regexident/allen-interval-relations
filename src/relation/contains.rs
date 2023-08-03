/// Methods for checking for a "contains" relation between intervals.
pub trait Contains<T>: Sized {
    /// Returns `true` iff `self` contains `other.0.
    ///
    /// ```plain
    /// self:  ┌────────────────┐
    /// other:     └────────┘
    /// ```
    #[inline]
    fn contains(&self, _other: &T) -> bool {
        false
    }

    /// Returns `true` iff `self` is contained by `other.0.
    ///
    /// ```plain
    /// self:      ┌────────┐
    /// other: └────────────────┘
    /// ```
    #[inline]
    fn is_contained_by(&self, other: &T) -> bool
    where
        T: Contains<Self>,
    {
        other.contains(self)
    }
}
