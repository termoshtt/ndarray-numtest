
use std::marker::PhantomData;
use rand::Rng;
use rand::distributions::*;
use ndarray::{DataOwned, Dimension, ArrayBase, ShapeBuilder};
use ndarray_rand::RandomExt;
use num_complex::Complex;
use num_traits::Num;

pub trait RandomExtNormal<S, D>: RandomExt<S, D>
    where S: DataOwned,
          D: Dimension
{
    fn random_normal<Sh>(shape: Sh, mean: S::Elem, variance: S::Elem) -> ArrayBase<S, D> where Sh: ShapeBuilder<Dim = D>;
}

pub trait RandomExtNormalComplex<A, S, D>: RandomExt<S, D>
    where S: DataOwned<Elem = Complex<A>>,
          D: Dimension
{
    fn random_normal_complex<Sh>(shape: Sh, mean: A, variance: A) -> ArrayBase<S, D> where Sh: ShapeBuilder<Dim = D>;
}

pub trait RandomExtRange<S, D>: RandomExt<S, D>
    where S: DataOwned,
          D: Dimension
{
    fn random_range<Sh, IdS>(shape: Sh, min: S::Elem, ma: S::Elem) -> ArrayBase<S, D> where Sh: ShapeBuilder<Dim = D>;
}

impl<A, S, D> RandomExtNormal<S, D> for ArrayBase<S, D>
    where S: DataOwned<Elem = A>,
          D: Dimension,
          A: From<f64> + Into<f64>
{
    fn random_normal<Sh>(shape: Sh, mean: A, variance: A) -> ArrayBase<S, D>
        where Sh: ShapeBuilder<Dim = D>
    {
        let dist = NormalAny::new(mean, variance);
        Self::random(shape, dist)
    }
}

impl<A, S, D> RandomExtNormalComplex<A, S, D> for ArrayBase<S, D>
    where S: DataOwned<Elem = Complex<A>>,
          D: Dimension,
          A: From<f64> + Into<f64> + Clone + Num
{
    fn random_normal_complex<Sh>(shape: Sh, mean: A, variance: A) -> ArrayBase<S, D>
        where Sh: ShapeBuilder<Dim = D>
    {
        let dist = ComplexNormal::new(mean, variance);
        Self::random(shape, dist)
    }
}

impl<A, S, D> RandomExtRange<S, D> for ArrayBase<S, D>
    where S: DataOwned<Elem = A>,
          D: Dimension,
          A: range::SampleRange + PartialOrd
{
    fn random_range<Sh, IdS>(shape: Sh, min: A, max: A) -> ArrayBase<S, D>
        where Sh: ShapeBuilder<Dim = D>
    {
        let dist = Range::new(min, max);
        Self::random(shape, dist)
    }
}

pub struct NormalAny<A> {
    dist: Normal,
    phantom: PhantomData<A>,
}

impl<A: From<f64> + Into<f64>> NormalAny<A> {
    pub fn new(center: A, var: A) -> Self {
        NormalAny {
            dist: Normal::new(center.into(), var.into()),
            phantom: PhantomData,
        }
    }
}

impl<A: From<f64> + Into<f64>> Sample<A> for NormalAny<A> {
    fn sample<R>(&mut self, rng: &mut R) -> A
        where R: Rng
    {
        self.dist.sample(rng).into()
    }
}

impl<A: From<f64> + Into<f64>> IndependentSample<A> for NormalAny<A>
    where f64: Into<A>
{
    fn ind_sample<R>(&self, rng: &mut R) -> A
        where R: Rng
    {
        self.dist.ind_sample(rng).into()
    }
}

pub struct ComplexNormal<A> {
    dist: Normal,
    phantom: PhantomData<A>,
}

impl<A: Into<f64>> ComplexNormal<A> {
    pub fn new(center: A, var: A) -> Self {
        ComplexNormal {
            dist: Normal::new(center.into(), var.into()),
            phantom: PhantomData,
        }
    }
}

impl<A> Sample<Complex<A>> for ComplexNormal<A>
    where A: Clone + Num + From<f64>
{
    fn sample<R>(&mut self, rng: &mut R) -> Complex<A>
        where R: Rng
    {
        let re = self.dist.sample(rng).into();
        let im = self.dist.sample(rng).into();
        Complex::new(re, im)
    }
}

impl<A> IndependentSample<Complex<A>> for ComplexNormal<A>
    where A: Clone + Num + From<f64>
{
    fn ind_sample<R>(&self, rng: &mut R) -> Complex<A>
        where R: Rng
    {
        let re = self.dist.ind_sample(rng).into();
        let im = self.dist.ind_sample(rng).into();
        Complex::new(re, im)
    }
}
