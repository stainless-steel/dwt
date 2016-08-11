//! [Discrete wavelet transform][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_wavelet_transform

// The implementation is based on:
// http://www.gnu.org/software/gsl/manual/html_node/Wavelet-Transforms.html

mod float;

pub mod wavelet;

use wavelet::Wavelet;

pub use float::Float;

/// A transform operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    /// The forward transform.
    Forward,
    /// The inverse transform.
    Inverse,
}

/// Perform the transform.
///
/// The number of points should be divisible by `2^level`. If the operation is
/// forward, the data are replaced by the approximation and detail coefficients
/// stored in the first and second halves of `data`, respectively. If the
/// operation is inverse, the data are assumed to be stored according to the
/// above convention.
pub fn transform<T>(data: &mut [T], operation: Operation, wavelet: &Wavelet<T>, level: usize)
    where T: Float
{
    if level == 0 {
        return;
    }
    let n = data.len();
    assert!(n % (1 << level) == 0);
    let mut work = Vec::with_capacity(n);
    unsafe { work.set_len(n) };
    match operation {
        Operation::Forward => {
            for i in 0..level {
                forward_step(data, wavelet, n >> i, &mut work);
            }
        },
        Operation::Inverse => {
            for i in 0..level {
                inverse_step(data, wavelet, n >> (level - i - 1), &mut work);
            }
        },
    }
}

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

#[inline(always)]
fn forward_step<T>(data: &mut [T], wavelet: &Wavelet<T>, n: usize, work: &mut [T])
    where T: Float
{
    zero!(work);
    let nm = wavelet.length * n - wavelet.offset;
    let nh = n >> 1;
    for i in 0..nh {
        let (mut h, mut g) = (T::zero(), T::zero());
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
fn inverse_step<T>(data: &mut [T], wavelet: &Wavelet<T>, n: usize, work: &mut [T])
    where T: Float
{
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
