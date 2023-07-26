/// Numbers which have upper and lower bounds.
///
/// **Important:** Unlike [num-traits][crates-io]' [Bounded][docs-rs] trait
/// this one accepts infinite min/max values.
///
/// [crates-io]: https://crates.io/crates/num-traits
/// [docs-rs]: https://docs.rs/num-traits/latest/num_traits/bounds/trait.Bounded.html
pub trait Bounded {
    /// Returns the smallest number this type can represent.
    fn min_value() -> Self;

    /// Returns the smallest number this type can represent.
    fn max_value() -> Self;
}

macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }

            #[inline]
            fn max_value() -> $t {
                $max
            }
        }
    };
}

bounded_impl!(usize, usize::MIN, usize::MAX);
bounded_impl!(u8, u8::MIN, u8::MAX);
bounded_impl!(u16, u16::MIN, u16::MAX);
bounded_impl!(u32, u32::MIN, u32::MAX);
bounded_impl!(u64, u64::MIN, u64::MAX);
bounded_impl!(u128, u128::MIN, u128::MAX);

bounded_impl!(isize, isize::MIN, isize::MAX);
bounded_impl!(i8, i8::MIN, i8::MAX);
bounded_impl!(i16, i16::MIN, i16::MAX);
bounded_impl!(i32, i32::MIN, i32::MAX);
bounded_impl!(i64, i64::MIN, i64::MAX);
bounded_impl!(i128, i128::MIN, i128::MAX);

bounded_impl!(f32, f32::NEG_INFINITY, f32::INFINITY);
bounded_impl!(f64, f64::NEG_INFINITY, f64::INFINITY);
