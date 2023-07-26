pub trait FromRanges<S, T>: Sized {
    fn from_ranges(s: S, t: T) -> Option<Self>;
}
