
use ndarray::{Array, Dimension, ShapeBuilder};
use ndarray_rand::RandomExt;
use num_complex::Complex;
use super::random;

pub trait RealNormalInit<D: Dimension> {
    fn real_normal_init<Sh>(shape: Sh, mean: f64, var: f64) -> Self where Sh: ShapeBuilder<Dim = D>;
}

pub trait ComplexNormalInit<D: Dimension> {
    fn complex_normal_init<Sh>(shape: Sh, re0: f64, im0: f64, re_var: f64, im_var: f64) -> Self
        where Sh: ShapeBuilder<Dim = D>;
}

macro_rules! impl_normal_init {
    ($float:ty) => {
impl<D> RealNormalInit<D> for Array<$float, D>
    where D: Dimension
{
    fn real_normal_init<Sh>(shape: Sh, mean: f64, var: f64) -> Self
        where Sh: ShapeBuilder<Dim = D>
    {
        let dist = random::RealNormal::new(mean, var);
        Array::random(shape, dist)
    }
}

impl<D> ComplexNormalInit<D> for Array<Complex<$float>, D>
    where D: Dimension
{
    fn complex_normal_init<Sh>(shape: Sh, re0: f64, im0: f64, re_var: f64, im_var: f64) -> Self
        where Sh: ShapeBuilder<Dim = D>
    {
        let dist = random::ComplexNormal::new(re0, im0, re_var, im_var);
        Array::random(shape, dist)
    }
}
}} // impl_normal_init

impl_normal_init!(f32);
impl_normal_init!(f64);
