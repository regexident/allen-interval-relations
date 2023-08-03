/// Methods for checking for a "overlaps" relation between intervals.
pub trait Overlaps<T>: Sized {
    /// Returns `true` iff `self` overlaps `other.0.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other:      └────────┘
    /// ```
    #[inline]
    fn overlaps(&self, _other: &T) -> bool {
        false
    }

    /// Returns `true` iff `self` is overlapped by `other.0.
    ///
    /// ```plain
    /// self:       ┌────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn is_overlapped_by(&self, other: &T) -> bool
    where
        T: Overlaps<Self>,
    {
        other.overlaps(self)
    }
}
