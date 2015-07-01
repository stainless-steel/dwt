//! Wavelets.

/// A wavelet.
pub struct Wavelet {
    /// The number of coefficients.
    pub length: usize,
    /// The offset of the coefficients.
    pub offset: usize,

    /// The coefficients of the decomposition low-pass filter.
    pub dec_lo: Vec<f64>,
    /// The coefficients of the decomposition high-pass filter.
    pub dec_hi: Vec<f64>,

    /// The coefficients of the reconstruction low-pass filter.
    pub rec_lo: Vec<f64>,
    /// The coefficients of the reconstruction high-pass filter.
    pub rec_hi: Vec<f64>,
}

/// A Haar wavelet.
pub struct Haar;

impl Haar {
    /// Create a wavelet.
    pub fn new() -> Wavelet {
        use std::f64::consts::FRAC_1_SQRT_2;
        Wavelet {
            length: 2,
            offset: 0,
            dec_lo: vec![FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            dec_hi: vec![FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
            rec_lo: vec![FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            rec_hi: vec![FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
        }
    }
}
