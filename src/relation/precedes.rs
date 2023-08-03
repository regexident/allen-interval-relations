/// Methods for checking for a "precedes" relation between intervals.
pub trait Precedes<T>: Sized {
    /// Returns `true` iff `self` precedes `other.0.
    ///
    /// ```plain
    /// self:  ┌────────┐
    /// other:            └────────┘
    /// ```
    #[inline]
    fn precedes(&self, _other: &T) -> bool {
        false
    }

    /// Returns `true` iff `self` is preceded by `other.0.
    ///
    /// ```plain
    /// self:             ┌────────┐
    /// other: └────────┘
    /// ```
    #[inline]
    fn is_preceded_by(&self, other: &T) -> bool
    where
        T: Precedes<Self>,
    {
        other.precedes(self)
    }
}
