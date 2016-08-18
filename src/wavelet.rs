//! Wavelets.

use num::{Float, FloatConst};

/// A wavelet.
pub struct Wavelet<T> {
    /// The number of coefficients.
    pub length: usize,
    /// The offset of the coefficients.
    pub offset: usize,

    /// The coefficients of the decomposition low-pass filter.
    pub dec_lo: Vec<T>,
    /// The coefficients of the decomposition high-pass filter.
    pub dec_hi: Vec<T>,

    /// The coefficients of the reconstruction low-pass filter.
    pub rec_lo: Vec<T>,
    /// The coefficients of the reconstruction high-pass filter.
    pub rec_hi: Vec<T>,
}

/// A Haar wavelet.
pub struct Haar;

impl Haar {
    /// Create a wavelet.
    pub fn new<T>() -> Wavelet<T> where T: Float + FloatConst {
        let value = T::FRAC_1_SQRT_2();
        Wavelet {
            length: 2,
            offset: 0,
            dec_lo: vec![value,  value],
            dec_hi: vec![value, -value],
            rec_lo: vec![value,  value],
            rec_hi: vec![value, -value],
        }
    }
}
