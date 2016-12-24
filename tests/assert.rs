
extern crate ndarray;
extern crate num_complex;
extern crate ndarray_numtest;

use ndarray::prelude::*;
use ndarray_numtest::prelude::*;
use num_complex::Complex;

#[allow(non_camel_case_types)]
type c64 = Complex<f64>;

#[test]
fn close_success() {
    let a = 1.0;
    let b = 1.0 + 1.0e-7;
    a.assert_close(b, 1e-5);
}

#[should_panic]
#[test]
fn close_fail() {
    let a = 1.0;
    let b = 1.0 + 1.0e-5;
    a.assert_close(b, 1e-7);
}

#[test]
fn close32_success() {
    let a = 1.0_f32;
    let b = 1.0_f32 + 1.0e-7;
    a.assert_close(b, 1e-5);
}

#[should_panic]
#[test]
fn close32_fail() {
    let a = 1.0_f32;
    let b = 1.0_f32 + 1.0e-2;
    a.assert_close(b, 1.0e-3);
}

#[test]
fn allclose_success() {
    let a = arr1(&[1.0, 2.0]);
    let b = arr1(&[1.0, 2.0 + 1.0e-9]);
    b.assert_allclose(&a, 1e-7);
}

#[test]
fn allclose_vec_success() {
    let a = vec![1.0, 2.0];
    let b = vec![1.0, 2.0 + 1.0e-9];
    b.assert_allclose(&a, 1e-7);
}

#[should_panic]
#[test]
fn allclose_fail() {
    let a = arr1(&[1.0, 2.0]);
    let b = arr1(&[1.0, 2.0 + 1.0e-3]);
    b.assert_allclose(&a, 1e-7);
}

#[should_panic]
#[test]
fn allclose_vec_fail() {
    let a = vec![1.0, 2.0];
    let b = vec![1.0, 2.0 + 1.0e-3];
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
