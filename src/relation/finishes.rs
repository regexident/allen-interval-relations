/// Methods for checking for a "finishes" relation between intervals.
pub trait Finishes<T>: Sized {
    /// Returns `true` iff `self` finishes `other.0.
    ///
    /// ```plain
    /// self:          ┌────────┐
    /// other: └────────────────┘
    /// ```
    #[inline]
    fn finishes(&self, _other: &T) -> bool {
        false
    }

    /// Returns `true` iff `self` is finished by `other.0.
    ///
    /// ```plain
    /// self:  ┌────────────────┐
    /// other:         └────────┘
    /// ```
    #[inline]
    fn is_finished_by(&self, other: &T) -> bool
    where
        T: Finishes<Self>,
    {
        other.finishes(self)
    }
}
