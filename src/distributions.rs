
use std::marker::PhantomData;
use rand::Rng;
use rand::distributions::*;
use num_complex::Complex;

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
