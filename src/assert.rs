//! Assertions for value and array

use ndarray::prelude::*;
use float_cmp::ApproxEqRatio;
use num_complex::Complex;

fn max<A: PartialOrd>(a: A, b: A) -> A {
    if a > b { a } else { b }
}

/// test two values are close
pub trait AssertClose: Sized + Copy {
    type Tol;
    fn assert_close(self, truth: Self, rtol: Self::Tol);
}

macro_rules! impl_AssertClose {
    ($scalar:ty) => {
impl AssertClose for $scalar {
    type Tol = $scalar;
    fn assert_close(self, truth: Self, rtol: Self::Tol) {
        if !self.approx_eq_ratio(&truth, rtol) {
            panic!("Not close: val={}, truth={}, rtol={}", self, truth, rtol);
        }
    }
}
impl AssertClose for Complex<$scalar> {
    type Tol = $scalar;
    fn assert_close(self, truth: Self, rtol: Self::Tol) {
        if !(self.re.approx_eq_ratio(&truth.re, rtol) && self.im.approx_eq_ratio(&truth.im, rtol)) {
            panic!("Not close: val={}, truth={}, rtol={}", self, truth, rtol);
        }
    }
}
}} // impl_AssertClose
impl_AssertClose!(f64);
impl_AssertClose!(f32);

/// test two arrays are close in maximum norm
pub trait AssertAllClose: Sized {
    type Tol;
    fn assert_allclose(&self, truth: &Self, rtol: Self::Tol);
}

macro_rules! impl_AssertAllClose {
    ($scalar:ty, $float:ty, $abs:ident, $th:expr) => {
impl AssertAllClose for Vec<$scalar> {
    type Tol = $float;
    fn assert_allclose(&self, truth: &Self, rtol: Self::Tol) {
        for (x, y) in self.iter().zip(truth.iter()) {
            let max_abs = max(x.$abs(), y.$abs());
            let diff_abs = (x - y).$abs();
            let tol = if max_abs < $th {
                diff_abs
            } else {
                diff_abs / max_abs
            };
            if tol > rtol {
                panic!("Not close (rtol={}): \ntest = \n{:?}\nTruth = \n{:?}", rtol, self, truth);
            }
        }
    }
}
impl<D: Dimension> AssertAllClose for Array<$scalar, D> {
    type Tol = $float;
    fn assert_allclose(&self, truth: &Self, rtol: Self::Tol) {
        for (x, y) in self.iter().zip(truth.iter()) {
            let max_abs = max(x.$abs(), y.$abs());
            let diff_abs = (x - y).$abs();
            let tol = if max_abs < $th {
                diff_abs
            } else {
                diff_abs / max_abs
            };
            if tol > rtol {
                panic!("Not close (rtol={}): \ntest = \n{:?}\nTruth = \n{:?}", rtol, self, truth);
            }
        }
    }
}
}} // impl_AssertAllClose

impl_AssertAllClose!(f64, f64, abs, 1.0e-7);
impl_AssertAllClose!(f32, f32, abs, 1.0e-3);
impl_AssertAllClose!(Complex<f64>, f64, norm, 1.0e-7);
impl_AssertAllClose!(Complex<f32>, f32, norm, 1.0e-3);
