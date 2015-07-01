//! Wavelets.

/// A wavelet.
pub struct Wavelet {
    /// The number of coefficients.
    pub length: usize,
    /// The offset of the coefficients.
    pub offset: usize,

    /// The coefficients of the decomposition low-pass filter.
    pub lo_dec: Vec<f64>,
    /// The coefficients of the decomposition high-pass filter.
    pub hi_dec: Vec<f64>,

    /// The coefficients of the reconstruction low-pass filter.
    pub lo_rec: Vec<f64>,
    /// The coefficients of the reconstruction high-pass filter.
    pub hi_rec: Vec<f64>,
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
            lo_dec: vec![FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            hi_dec: vec![FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
            lo_rec: vec![FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            hi_rec: vec![FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
        }
    }
}
