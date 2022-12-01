use std::{fmt::{Debug}, ops::{Mul, AddAssign, Add, Neg, Sub, Div}, iter::Sum, process::Output};

use num::traits::Pow;

use crate::tensor::utils::alloc_box_buffer;

pub mod utils;

pub mod macros;

pub mod ops;

pub trait TensorTrait<T>: Debug +  Copy  + Add + Sum + Mul + AddAssign + num::Float  + 'static
{
    type T;
    fn default() -> Self::T;

}

impl <T> TensorTrait<T> for f32 {
    type T = f32;
    fn default() -> f32 {
        0.0
    }

}

impl <T> TensorTrait<T> for f64 {
    type T = f64;
    fn default() -> f64 {
        0.0
    }
}



#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct Tensor<T: TensorTrait<T>>
where
    T : Copy
{
    pub inner      : Box<[T]>,

    pub shape      : Box<[usize]>,

}

#[allow(dead_code)]
impl <T: TensorTrait<T>> Tensor<T> {
    pub fn new(data: Box<[T]>, shape: Box<[usize]>) -> Self {
        Self {
            inner  : data,
            shape,
        }
    }

    pub fn add(& mut self, rhs: Tensor<T>) {
        assert_eq!(self.shape, rhs.shape);

        self.inner.iter_mut().enumerate().for_each(|(index, inner_val)| *inner_val += rhs.inner[index])
    }

}

impl <T: TensorTrait<T> + Mul<Output=T>> Mul<T> for &Tensor<T>
where
{
    type Output = Tensor<T>;

    fn mul(self, rhs: T) -> Tensor<T> {

        Tensor {
            inner: self.inner.iter().map(|inner_value| *inner_value * rhs).collect::<Vec<_>>().into_boxed_slice(),
            shape: self.shape.clone()
        }
    }
}

impl <T: TensorTrait<T> + Mul<Output=T>> Mul<T> for Tensor<T>
where
{
    type Output = Tensor<T>;

    fn mul(self, rhs: T) -> Tensor<T> {

        Tensor {
            inner: self.inner.iter().map(|inner_value| *inner_value * rhs).collect::<Vec<_>>().into_boxed_slice(),
            shape: self.shape.clone()
        }
    }
}

impl <T: TensorTrait<T> + Add<Output=T>> Add<Tensor<T>> for &Tensor<T>
where {

    type Output = Tensor<T>;

    fn add(self, rhs: Tensor<T>) -> Tensor<T> {
        assert_eq!(self.shape, rhs.shape);

        let mut new_inner = alloc_box_buffer::<T>(self.inner.len());

        for (index, inner) in new_inner.iter_mut().enumerate() {
            *inner = self.inner[index] + rhs.inner[index]
        }

        Tensor {
            inner: new_inner,
            shape: self.shape.clone(),
        }

    }
}

impl <T: TensorTrait<T> + Add<Output=T>> Add<Self> for Tensor<T>
where {

    type Output = Self;

    fn add(self, rhs: Self) -> Tensor<T> {
        assert_eq!(self.shape, rhs.shape);

        let mut new_inner = alloc_box_buffer::<T>(self.inner.len());

        for (index, inner) in new_inner.iter_mut().enumerate() {
            *inner = self.inner[index] + rhs.inner[index]
        }

        Tensor {
            inner: new_inner,
            shape: self.shape.clone(),
        }

    }
}

impl <T: TensorTrait<T> + Add<Output=T>> Add<T> for &Tensor<T>
where {

    type Output = Tensor<T>;

    fn add(self, rhs: T) -> Tensor<T> {
        let mut new_inner = alloc_box_buffer::<T>(self.inner.len());

        new_inner.iter_mut().enumerate().for_each(|(index, inner_val)| *inner_val = self.inner[index] + rhs);

        Tensor {
            inner: new_inner,
            shape: self.shape.clone(),
        }

    }
}