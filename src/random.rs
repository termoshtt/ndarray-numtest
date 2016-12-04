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
