
extern crate ndarray;
extern crate num;
extern crate ndarray_numtest;

use ndarray::prelude::*;
use ndarray_numtest::prelude::*;
use num::complex::Complex;

#[allow(non_camel_case_types)]
type c64 = Complex<f64>;

#[test]
fn allclose_success() {
    let a = arr1(&[1.0, 2.0]);
    let b = arr1(&[1.0, 2.0 + 1.0e-9]);
    b.assert_allclose(&a, 1e-7);
}

#[should_panic]
#[test]
fn allclose_fail() {
    let a = arr1(&[1.0, 2.0]);
    let b = arr1(&[1.0, 2.0 + 1.0e-3]);
    b.assert_allclose(&a, 1e-7);
}

#[test]
fn allclose_success_complex() {
    let a = arr1(&[c64::new(1.0, 1.1), c64::new(1.0, 1.1)]);
    let b = arr1(&[c64::new(1.0, 1.1 + 1.0e-9), c64::new(1.0, 1.1)]);
    b.assert_allclose(&a, 1e-7);
}

#[should_panic]
#[test]
fn allclose_success_fail() {
    let a = arr1(&[c64::new(1.0, 1.1), c64::new(1.0, 1.1)]);
    let b = arr1(&[c64::new(1.0, 1.1 + 1.0e-3), c64::new(1.0, 1.1)]);
    b.assert_allclose(&a, 1e-7);
}
