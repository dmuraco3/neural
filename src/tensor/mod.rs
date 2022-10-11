use std::{fmt::{Debug}};

pub mod utils;

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



// create tensor up to 10 dimensions
// from https://docs.rs/ndarray/latest/ndarray/macro.array.html

#[macro_export]
macro_rules! tensor {
    (
        //10
        $([$([$([$([$([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*

    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*],)*],)*],)*],)*];

            let dims = [
                t[0][0][0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0].len(),
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                10
            ]

        }
    }};
    (
        //9
        $([$([$([$([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*

    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*],)*],)*],)*];
            
            let dims = [
                t[0][0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0].len(),
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                9
            ]

        }
    }};
    (
        //8
        $([$([$([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*],)*],)*];

            let dims = [
                t[0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0].len(),
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                8
            ]
        }
    }};
    (
        //7
        $([$([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*],)*];

            let dims = [
                t[0][0][0][0][0][0].len(),
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                7
            ]

        }
    }};
    (
        //6
        $([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*];
            let dims = [
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                6
            ]
        }
    }};
    (
        //5
        $([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*];
            let dims = [
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                5
            ]
        }
    }};
    (
        //4
        $([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*];
            let dims = [
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                4
            ]
        }
    }};
    (
        //3
        $d_type: ty,
        [$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]
    ) => {{
        {
            let t = [$([$([$($x,)*],)*],)*];

            let dims = [
                t[0][0].len(),
                t[0].len(),
                3
            ];

            let dim_prod = dims.iter().fold(1, |sum, val| sum * val);


            let p = t.iter().flat_map(|x| x.iter()).flat_map(|x| x.iter()).cloned().collect::<Vec<$d_type>>();



            "bruh"

        }
    }};
    (
        //2
        $dtype: ty,
        [$([$($x:expr),* $(,)*]),+ $(,)*]
    ) => {{
        {
            let t = vec![$([$($x,)*],)*];

            let dims = [
                t[0].len(),
                2
            ];
            
            let dim_prod = dims.iter().fold(1, |sum, val| sum * val);



        }
    }};
    (
        //1
        $dtype: ty,
        [$($x:expr),* $(,)*]
    ) => {{
        {
            let t = vec![$($x,)*];
            let dims = [t.len()];
        }

    }};
    
}
#[macro_export]
macro_rules! new_from_matrix {
    ($dtype: ty, $y: expr) => {{

        use $crate::tensor::utils::RecursivelyFlattenIterator;
        $y.into_iter().recursively_flatten::<_, i32>().collect::<Vec<$dtype>>()
    }};
}

// code from https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4d4c86f7f5d2c77ac73ca1de5dde8c4b from author https://users.rust-lang.org/u/steffahn

