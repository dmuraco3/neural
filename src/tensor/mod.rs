use std::{fmt::{Debug}, ops::{Mul, Add}, iter::Sum};

pub mod utils;

pub mod macros;

pub mod ops;

pub trait TensorTrait<T>: Debug +  Copy  + 'static {
    type T;
    fn default() -> Self::T;
}

impl <T> TensorTrait<T> for i32 {
    type T = i32;
    fn default() -> i32
    {
        0
    }
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
}
