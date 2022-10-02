use std::fmt::{Display, Debug, Formatter, Result};
pub trait TensorTrait<T>: Debug {}
impl<T> TensorTrait<T> for T where T: Debug {}


#[allow(dead_code)]
pub struct Tensor<T: TensorTrait<T>> {
    pub data: Vec<T>,
    pub left_parent: Option<Box<Tensor<T>>>,
    pub right_parent: Option<Box<Tensor<T>>>,
    pub shape: (usize, usize, usize),
    pub ndim: usize,


}


#[allow(dead_code)]
impl<T: TensorTrait<T>> Tensor<T>  {
    pub fn new(data: Vec<T>, shape: (usize, usize), ndim: usize, left_parent: Option<Box<Tensor<T>>>, right_parent: Option<Box<Tensor<T>>>) -> Self {
        Tensor {
            data,
            left_parent,
            right_parent,
            shape,
            ndim,
        }
    }

    // either figure this out or have programmer add shape in struct construction 


}


impl<T: TensorTrait<T>> Display for Tensor<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.data)
    }
}

// impl<T: TensorTrait<T>> Add for Tensor<T> {
//     type Output = Self;
//     fn add(Self, other: Self) -> Self {
        
//     }
// }

// from https://docs.rs/ndarray/latest/ndarray/macro.array.html
#[macro_export]
macro_rules! tensor {
    ($([$([$($x:expr),* $(,)*]),+ $(,)*]),+ $(,)*) => {{
        {
            let t = vec![$([$([$($x,)*],)*],)*];
            let shape = (t.len() , t[0].len(), 3);
            crate::tensor::Tensor::new(t, shape, 3, None, None)
        }
    }};
    ($([$($x:expr),* $(,)*]),+ $(,)*) => {{
        {
            let t = vec![$([$($x,)*],)*];
            let shape = (t.len() , t[0].len(), 2);
            crate::tensor::Tensor::new(t, shape, 2, None, None)
        }
    }};
    ($($x:expr),* $(,)*) => {{
        {
            let t = vec![$($x,)*];
            let shape = (t.len(), 0, 0);
            crate::tensor::Tensor::new(t, shape, 1, None, None)
        }

    }};
    ($($x: expr)) => {{
        {}
    }}
}