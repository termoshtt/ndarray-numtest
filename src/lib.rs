extern crate ndarray;
extern crate float_cmp;

use ndarray::prelude::*;
use float_cmp::ApproxEqRatio;

fn max(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

pub fn assert_close(val: f64, truth: f64, rtol: f64) {
    if !val.approx_eq_ratio(&truth, rtol) {
        panic!("Not close: val={:?}, truth={:?}, rtol={:?}",
               val,
               truth,
               rtol);
    }
}

/// assert to check `test` and `truth` are close in maximum norm
pub fn assert_allclose<D: Dimension>(test: &Array<f64, D>, truth: &Array<f64, D>, rtol: f64) {
    for (x, y) in test.iter().zip(truth.iter()) {
        let max_abs = max(x.abs(), y.abs());
        let diff_abs = (x - y).abs();
        let tol = if max_abs < 1.0e-7 {
            diff_abs
        } else {
            diff_abs / max_abs
        };
        if tol > rtol {
            panic!("Not close (rtol={}): \ntest = \n{:?}\nTruth = \n{:?}",
                   rtol,
                   test,
                   truth);
        }
    }
}
