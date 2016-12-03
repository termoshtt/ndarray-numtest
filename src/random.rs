
use rand;
use rand::distributions::*;
use ndarray_rand::RandomExt;

pub trait RandomInit: Sized {
    type Scalar;
    type Size;
    /// Generate new array filled with random variables from Gaussian
    fn random_normal(size: Self::Size, center: Self::Scalar, variance: Self::Scalar) -> Self;
    /// Generate new array filled with random variables from uniform distribution
    fn random_range(size: Self::Size, min: Self::Scalar, max: Self::Scalar) -> Self;
}

impl RandomInit for Vec<f64> {
    type Size = usize;
    type Scalar = f64;
    fn random_normal(size: Self::Size, center: Self::Scalar, variance: Self::Scalar) -> Self {
        let dist = Normal::new(center, variance);
        let mut rng = rand::thread_rng();
        (0..size).map(|_| dist.ind_sample(&mut rng)).collect()
    }
    fn random_range(size: Self::Size, min: Self::Scalar, max: Self::Scalar) -> Self {
        let dist = Range::new(min, max);
        let mut rng = rand::thread_rng();
        (0..size).map(|_| dist.ind_sample(&mut rng)).collect()
    }
}

impl RandomInit for Vec<f32> {
    type Size = usize;
    type Scalar = f32;
    fn random_normal(size: Self::Size, center: Self::Scalar, variance: Self::Scalar) -> Self {
        let dist = Normal::new(center, variance);
        let mut rng = rand::thread_rng();
        (0..size).map(|_| dist.ind_sample(&mut rng)).collect()
    }
    fn random_range(size: Self::Size, min: Self::Scalar, max: Self::Scalar) -> Self {
        let dist = Range::new(min, max);
        let mut rng = rand::thread_rng();
        (0..size).map(|_| dist.ind_sample(&mut rng)).collect()
    }
}
