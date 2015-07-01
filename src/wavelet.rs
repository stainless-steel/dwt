//! Wavelets.

/// A wavelet.
pub struct Wavelet {
    pub length: usize,
    pub offset: usize,
    pub h1: Vec<f64>,
    pub g1: Vec<f64>,
    pub h2: Vec<f64>,
    pub g2: Vec<f64>,
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
            h1: vec![FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            g1: vec![FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
            h2: vec![FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            g2: vec![FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
        }
    }
}
