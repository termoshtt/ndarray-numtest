
extern crate ndarray;
extern crate ndarray_rand;
extern crate ndarray_numtest;

use ndarray::prelude::*;
use ndarray_rand::*;
use ndarray_numtest::prelude::*;

#[test]
fn random() {
    let dist64 = RealNormal::<f64>::new(1.0, 0.1);
    let a = Array::random(12, dist64);
    let dist32 = RealNormal::<f32>::new(1.0, 0.1);
    let b = Array::random((12, 3), dist32);
    println!("a = \n{:?}", &a);
    println!("b = \n{:?}", &b);
}

#[test]
fn random_complex() {
    let dist = ComplexNormal::<f32>::new(1.0, 0.0, 0.1, 0.2);
    let a = Array::random(5, dist);
    println!("a = \n{:?}", &a);
}

#[test]
fn random_init() {
    let a = Array::<f64, _>::real_normal_init((3, 4), 0.0, 1.0);
    let b = Array::<f32, _>::real_normal_init((3, 4), 0.0, 1.0);
    println!("a = \n{:?}", &a);
    println!("b = \n{:?}", &b);
}
