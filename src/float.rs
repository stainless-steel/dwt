use std::ops::{Add, AddAssign, Mul, Neg};

/// A floating-point number.
pub trait Float: Copy + Add<Output=Self> + AddAssign + Mul<Output=Self> + Neg<Output=Self> {
    /// Return `1.0 / sqrt(2.0)`.
    fn frac_1_sqrt_2() -> Self;

    /// Return `0.0`.
    fn zero() -> Self;
}

macro_rules! implement {
    ($($kind:ident),*) => ($(
        impl Float for $kind {
            #[inline(always)]
            fn frac_1_sqrt_2() -> Self { ::std::$kind::consts::FRAC_1_SQRT_2 }

            #[inline(always)]
            fn zero() -> Self { 0.0 }
        }
    )*);
}

implement!(f32, f64);
