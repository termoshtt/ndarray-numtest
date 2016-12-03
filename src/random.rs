
use std::marker::PhantomData;
use rand::Rng;
use rand::distributions::*;
use ndarray::{DataOwned, Dimension, ArrayBase, ShapeBuilder};
use ndarray_rand::RandomExt;

pub trait RandomExtAppend<S, D>: RandomExt<S, D>
    where S: DataOwned,
          D: Dimension
{
    fn random_normal<Sh>(shape: Sh, mean: S::Elem, variance: S::Elem) -> ArrayBase<S, D> where Sh: ShapeBuilder<Dim = D>;
    fn random_range<Sh, IdS>(shape: Sh, min: S::Elem, ma: S::Elem) -> ArrayBase<S, D> where Sh: ShapeBuilder<Dim = D>;
}

impl<A, S, D> RandomExtAppend<S, D> for ArrayBase<S, D>
    where S: DataOwned<Elem = A>,
          D: Dimension,
          A: From<f64> + Into<f64> + range::SampleRange + PartialOrd
{
    fn random_normal<Sh>(shape: Sh, mean: A, variance: A) -> ArrayBase<S, D>
        where Sh: ShapeBuilder<Dim = D>
    {
        let dist = NormalAny::new(mean, variance);
        Self::random(shape, dist)
    }
    fn random_range<Sh, IdS>(shape: Sh, min: A, max: A) -> ArrayBase<S, D>
        where Sh: ShapeBuilder<Dim = D>
    {
        let dist = Range::new(min, max);
        Self::random(shape, dist)
    }
}

pub struct NormalAny<A: From<f64> + Into<f64>> {
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
