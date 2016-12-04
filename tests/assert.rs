
extern crate ndarray;
extern crate ndarray_numtest;

use ndarray::prelude::*;
use ndarray_numtest::prelude::*;

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
