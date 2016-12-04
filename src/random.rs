#![allow(unused_imports)]

use std::marker::PhantomData;
use rand::Rng;
use rand::distributions::*;
use ndarray::{DataOwned, Dimension, ArrayBase, ShapeBuilder};
use ndarray_rand::RandomExt;
use num_complex::Complex;
use num_traits::Num;

// pub trait RandomExtNormal<A, S, D>: RandomExt<S, D>
//     where S: DataOwned<Elem = A>,
//           D: Dimension
// {
//     fn random_normal<Sh>(shape: Sh, mean: A, variance: A) -> ArrayBase<S, D> where Sh: ShapeBuilder<Dim = D>;
// }
//
// pub trait RandomExtNormalComplex<A, S, D>: RandomExt<S, D>
//     where S: DataOwned<Elem = Complex<A>>,
//           D: Dimension
// {
//     fn random_normal_complex<Sh>(shape: Sh, mean: A, variance: A) -> ArrayBase<S, D> where Sh: ShapeBuilder<Dim = D>;
// }
//
// pub trait RandomExtRange<S, D>: RandomExt<S, D>
//     where S: DataOwned,
//           D: Dimension
// {
//     fn random_range<Sh, IdS>(shape: Sh, min: S::Elem, ma: S::Elem) -> ArrayBase<S, D> where Sh: ShapeBuilder<Dim = D>;
// }
//
// impl<A, S, D> RandomExtNormal<A, S, D> for ArrayBase<S, D>
//     where S: DataOwned<Elem = A>,
//           D: Dimension,
//           A: From<f64> + Into<f64>
// {
//     fn random_normal<Sh>(shape: Sh, mean: A, variance: A) -> ArrayBase<S, D>
//         where Sh: ShapeBuilder<Dim = D>
//     {
//         let dist = NormalAny::new(mean as f64, variance as f64);
//         Self::random(shape, dist)
//     }
// }
//
// impl<A, S, D> RandomExtNormalComplex<A, S, D> for ArrayBase<S, D>
//     where S: DataOwned<Elem = Complex<A>>,
//           D: Dimension,
//           A: From<f64> + Into<f64> + Clone + Num
// {
//     fn random_normal_complex<Sh>(shape: Sh, mean: A, variance: A) -> ArrayBase<S, D>
//         where Sh: ShapeBuilder<Dim = D>
//     {
//         let dist = ComplexNormal::new(mean, variance);
//         Self::random(shape, dist)
//     }
// }
//
// impl<A, S, D> RandomExtRange<S, D> for ArrayBase<S, D>
//     where S: DataOwned<Elem = A>,
//           D: Dimension,
//           A: range::SampleRange + PartialOrd
// {
//     fn random_range<Sh, IdS>(shape: Sh, min: A, max: A) -> ArrayBase<S, D>
//         where Sh: ShapeBuilder<Dim = D>
//     {
//         let dist = Range::new(min, max);
//         Self::random(shape, dist)
//     }
// }

#[derive(Clone, Copy)]
pub struct NormalAny<A> {
    dist: Normal,
    phantom: PhantomData<A>,
}

impl<A> NormalAny<A> {
    pub fn new(center: f64, var: f64) -> Self {
        NormalAny {
            dist: Normal::new(center, var),
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_NormalAny {
    ($float:ty) => {
impl Sample<$float> for NormalAny<$float> {
    fn sample<R>(&mut self, rng: &mut R) -> $float
        where R: Rng
    {
        self.dist.sample(rng) as $float
    }
}

impl IndependentSample<$float> for NormalAny<$float> {
    fn ind_sample<R>(&self, rng: &mut R) -> $float
        where R: Rng
    {
        self.dist.ind_sample(rng) as $float
    }
}
}} // impl_NormalAny

impl_NormalAny!(f64);
impl_NormalAny!(f32);

#[derive(Clone, Copy)]
pub struct ComplexNormal<A> {
    re_dist: Normal,
    im_dist: Normal,
    phantom: PhantomData<A>,
}

impl<A> ComplexNormal<A> {
    pub fn new(re0: f64, im0: f64, re_var: f64, im_var: f64) -> Self {
        ComplexNormal {
            re_dist: Normal::new(re0, re_var),
            im_dist: Normal::new(im0, im_var),
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_ComplexNormal {
    ($float:ty) => {
impl Sample<Complex<$float>> for ComplexNormal<$float> {
    fn sample<R>(&mut self, rng: &mut R) -> Complex<$float>
        where R: Rng
    {
        let re = self.re_dist.sample(rng) as $float;
        let im = self.im_dist.sample(rng) as $float;
        Complex::new(re, im)
    }
}

impl IndependentSample<Complex<$float>> for ComplexNormal<$float>
{
    fn ind_sample<R>(&self, rng: &mut R) -> Complex<$float>
        where R: Rng
    {
        let re = self.re_dist.ind_sample(rng) as $float;
        let im = self.im_dist.ind_sample(rng) as $float;
        Complex::new(re, im)
    }
}
}} // impl_ComplexNormal

impl_ComplexNormal!(f32);
impl_ComplexNormal!(f64);
