//! [Discrete wavelet transform][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_wavelet_transform

// The implementation is based on:
// http://www.gnu.org/software/gsl/manual/html_node/Wavelet-Transforms.html

pub mod wavelet;

use wavelet::Wavelet;

macro_rules! power_of_two(
    ($data:expr) => (match $data.len() {
        n if n < 2 => return,
        n if n & (n - 1) != 0 => panic!("expected the number of points to be a power of two"),
        n => n,
    });
);

macro_rules! dirty_buffer(
    ($n:expr) => ({
        let mut buffer = Vec::with_capacity($n);
        unsafe { buffer.set_len($n) };
        buffer
    });
);

macro_rules! copy(
    ($source:ident, $destination:ident, $n:expr) => ({
        use std::ptr::copy_nonoverlapping as copy;
        unsafe { copy($source.as_ptr(), $destination.as_mut_ptr(), $n) };
    });
);

macro_rules! zero(
    ($buffer:expr) => ({
        use std::ptr::write_bytes as write;
        unsafe { write($buffer.as_mut_ptr(), 0, $buffer.len()) };
    });
);

/// Perform the forward transformation.
///
/// The number of points should be a power of two.
pub fn forward(data: &mut [f64], wavelet: &Wavelet) {
    let n = power_of_two!(data);
    let mut work = dirty_buffer!(n);
    let mut i = n;
    while i >= 2 {
        forward_step(data, wavelet, i, &mut work);
        i >>= 1;
    }
}

/// Perform the inverse transformation.
///
/// The number of points should be a power of two.
pub fn inverse(data: &mut [f64], wavelet: &Wavelet) {
    let n = power_of_two!(data);
    let mut work = dirty_buffer!(n);
    let mut i = 2;
    while i <= n {
        inverse_step(data, wavelet, i, &mut work);
        i <<= 1;
    }
}

#[inline(always)]
fn forward_step(data: &mut [f64], wavelet: &Wavelet, n: usize, work: &mut [f64]) {
    zero!(work);
    let nmod = wavelet.length * n - wavelet.offset;
    let (n1, nh) = (n - 1, n >> 1);
    for i in 0..nh {
        let (mut h, mut g) = (0.0, 0.0);
        let n = 2 * i + nmod;
        for j in 0..wavelet.length {
            let k = n1 & (n + j);
            h += wavelet.h1[j] * data[k];
            g += wavelet.g1[j] * data[k];
        }
        work[i] += h;
        work[i + nh] += g;
    }
    copy!(work, data, n);
}

#[inline(always)]
fn inverse_step(data: &mut [f64], wavelet: &Wavelet, n: usize, work: &mut [f64]) {
    zero!(work);
    let nmod = wavelet.length * n - wavelet.offset;
    let (n1, nh) = (n - 1, n >> 1);
    for i in 0..nh {
        let (h, g) = (data[i], data[i + nh]);
        let n = 2 * i + nmod;
        for j in 0..wavelet.length {
            let k = n1 & (n + j);
            work[k] += wavelet.h2[j] * h + wavelet.g2[j] * g;
        }
    }
    copy!(work, data, n);
}
