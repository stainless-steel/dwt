//! [Discrete wavelet transform][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Discrete_wavelet_transform

// The implementation is based on:
// http://www.gnu.org/software/gsl/manual/html_node/Wavelet-Transforms.html

extern crate num_traits as num;

use num::Float;

mod transform;

pub mod wavelet;

use wavelet::Wavelet;

pub use transform::Transform;

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
/// The function is a shortcut for `Transform::transform`.
#[inline(always)]
pub fn transform<T>(data: &mut [T], operation: Operation, wavelet: &Wavelet<T>, level: usize)
    where T: Float
{
    Transform::transform(data, operation, wavelet, level);
}
