use num::Float;

use Operation;
use wavelet::Wavelet;

/// The transform.
pub trait Transform<T> {
    /// Perform the transform.
    ///
    /// The number of points should be divisible by `2^level`. If the operation
    /// is forward, the data are replaced by the approximation and detail
    /// coefficients stored in the first and second halves of `data`,
    /// respectively. If the operation is inverse, the data are assumed to be
    /// stored according to the above convention.
    fn transform(&mut self, operation: Operation, wavelet: &Wavelet<T>, level: usize);
}

impl<T> Transform<T> for [T] where T: Float {
    fn transform(&mut self, operation: Operation, wavelet: &Wavelet<T>, level: usize) {
        if level == 0 {
            return;
        }
        let n = self.len();
        assert!(n % (1 << level) == 0);
        let mut work = Vec::with_capacity(n);
        unsafe { work.set_len(n) };
        match operation {
            Operation::Forward => {
                for i in 0..level {
                    forward_step(self, wavelet, n >> i, &mut work);
                }
            },
            Operation::Inverse => {
                for i in 0..level {
                    inverse_step(self, wavelet, n >> (level - i - 1), &mut work);
                }
            },
        }
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
pub fn forward_step<T>(data: &mut [T], wavelet: &Wavelet<T>, n: usize, work: &mut [T])
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
            h = h + wavelet.dec_lo[j] * data[k];
            g = g + wavelet.dec_hi[j] * data[k];
        }
        work[i] = work[i] + h;
        work[i + nh] = work[i + nh] + g;
    }
    copy!(work, data, n);
}

#[inline(always)]
pub fn inverse_step<T>(data: &mut [T], wavelet: &Wavelet<T>, n: usize, work: &mut [T])
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
            work[k] = work[k] + wavelet.rec_lo[j] * h + wavelet.rec_hi[j] * g;
        }
    }
    copy!(work, data, n);
}
