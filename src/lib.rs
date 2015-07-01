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
    ($source:ident, $destination:ident, $n:expr) => (
        for i in 0..$n {
            $destination[i] = $source[i];
        }
    );
);

macro_rules! zero(
    ($buffer:expr) => ({
        for x in $buffer.iter_mut() {
            *x = 0.0;
        }
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
    let (mut i, mut ii) = (0, 0);
    while i < n {
        let (mut h, mut g) = (0.0, 0.0);
        let ni = i + nmod;
        for k in 0..wavelet.length {
            let jf = n1 & (ni + k);
            h += wavelet.h1[k] * data[jf];
            g += wavelet.g1[k] * data[jf];
        }
        work[ii] += h;
        work[ii + nh] += g;
        i += 2;
        ii += 1;
    }
    copy!(work, data, n);
}

#[inline(always)]
fn inverse_step(data: &mut [f64], wavelet: &Wavelet, n: usize, work: &mut [f64]) {
    zero!(work);
    let nmod = wavelet.length * n - wavelet.offset;
    let (n1, nh) = (n - 1, n >> 1);
    let (mut i, mut ii) = (0, 0);
    while i < n {
        let ai = data[ii];
        let ai1 = data[ii + nh];
        let ni = i + nmod;
        for k in 0..wavelet.length {
            let jf = n1 & (ni + k);
            work[jf] += wavelet.h2[k] * ai + wavelet.g2[k] * ai1;
        }
        i += 2;
        ii += 1;
    }
    copy!(work, data, n);
}
