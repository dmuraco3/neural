use std::{fmt::{Debug}};

pub mod utils;

pub mod macros;

pub trait TensorTrait<T>: Debug + 'static {}

impl <T> TensorTrait<T> for i32 {}

impl <T> TensorTrait<T> for f32 {}

impl <T> TensorTrait<T> for f64 {}



#[allow(dead_code)]
pub struct Tensor<T: TensorTrait<T>> {
    inner: Box<[T]>,

    shape: Box<[usize]>,

    grad: Option<f32>,

    bias: Option<f32>,

    weights: Option<&'static [f32]>
}
#[allow(dead_code)]
impl <T: TensorTrait<T>> Tensor<T> {
    pub fn new(data: Box<[T]>, dims: Box<[usize]>) -> Self {
        Self {
            inner: data,
            shape: dims,
            grad: None,
            bias: None,
            weights: None
        }
    }

}