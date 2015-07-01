//! [Discrete wavelet transform][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_wavelet_transform

// The implementation is based on:
// http://www.gnu.org/software/gsl/manual/html_node/Wavelet-Transforms.html

pub mod wavelet;

use wavelet::Wavelet;

macro_rules! power_of_two(
    ($data:expr, $level:expr) => ({
        if $level == 0 {
            return;
        }
        let n = $data.len();
        if n % (1 << $level) != 0 {
            panic!("expected the number of points to be divisible by 2^level");
        }
        n
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
/// The number of points should be divisible by `2^level`. The data are replaced
/// by the approximation and detail coefficients stored in the first and second
/// halves of `data`, respectively.
pub fn forward(data: &mut [f64], wavelet: &Wavelet, level: usize) {
    let n = power_of_two!(data, level);
    let mut work = dirty_buffer!(n);
    for i in 0..level {
        forward_step(data, wavelet, n >> i, &mut work);
    }
}

/// Perform the inverse transformation.
///
/// The number of points should be divisible by `2^level`. The approximation and
/// detail coefficients should be stored as described in `forward`.
pub fn inverse(data: &mut [f64], wavelet: &Wavelet, level: usize) {
    let n = power_of_two!(data, level);
    let mut work = dirty_buffer!(n);
    for i in 0..level {
        inverse_step(data, wavelet, n >> (level - i - 1), &mut work);
    }
}

#[inline(always)]
fn forward_step(data: &mut [f64], wavelet: &Wavelet, n: usize, work: &mut [f64]) {
    zero!(work);
    let nm = wavelet.length * n - wavelet.offset;
    let nh = n >> 1;
    for i in 0..nh {
        let (mut h, mut g) = (0.0, 0.0);
        let k = 2 * i + nm;
        for j in 0..wavelet.length {
            let k = (k + j) % n;
            h += wavelet.dec_lo[j] * data[k];
            g += wavelet.dec_hi[j] * data[k];
        }
        work[i] += h;
        work[i + nh] += g;
    }
    copy!(work, data, n);
}

#[inline(always)]
fn inverse_step(data: &mut [f64], wavelet: &Wavelet, n: usize, work: &mut [f64]) {
    zero!(work);
    let nm = wavelet.length * n - wavelet.offset;
    let nh = n >> 1;
    for i in 0..nh {
        let (h, g) = (data[i], data[i + nh]);
        let k = 2 * i + nm;
        for j in 0..wavelet.length {
            let k = (k + j) % n;
            work[k] += wavelet.rec_lo[j] * h + wavelet.rec_hi[j] * g;
        }
    }
    copy!(work, data, n);
}
