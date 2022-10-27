use std::{fmt::{Debug}, ops::{Mul, Add}};

pub mod utils;

pub mod macros;

pub trait TensorTrait<T>: Debug +  Copy +'static {}

impl <T> TensorTrait<T> for i32 {}

impl <T> TensorTrait<T> for f32 {}

impl <T> TensorTrait<T> for f64 {}



#[allow(dead_code)]
pub struct Tensor<T: TensorTrait<T>> {
    pub inner  : Box<[T]>,

    shape  : Box<[usize]>,

    // strides: Box<[usize]>,
}
#[allow(dead_code)]
impl <T: TensorTrait<T>> Tensor<T> {
    pub fn new(data: Box<[T]>, dims: Box<[usize]>) -> Self {


        // calculating and storing strides to save computation time at indexing
        let mut strides: Vec<usize> = Vec::new();

        for dim in 0..dims.len()-1 {
            let y = dims[dim+1..].iter().fold(1, |acc, x| acc * x);
            strides.push(y);
        }

        Self {
            inner  : data,
            shape  : dims,
            // strides: strides.into_boxed_slice(),
        }
    }


    // indexing in this library works like array accesses so that [x,y,z,...] are reversed [...,z,y,x]
    fn index(self, coordinates: &[usize])  -> T {
        if coordinates.len() == self.shape.len() {
            let mut sum = 0;



            for x in 0..self.shape.len()-1 {
                let y = self.shape[x+1..].iter().fold(1, |acc, x| acc * x);
                sum += y * coordinates[x]
            }
            sum += coordinates[coordinates.len()-1];
            self.inner[sum]

        } else {
            panic!("shape of coordinates does not match shape of tensor");
        }
    }


}

// impl <T: TensorTrait<T>> Mul for Tensor<T> {
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self {
//         if self.shape[0] == rhs.shape[rhs.shape.len()-1] {

//         } else {
//             // todo: better error message here
//             panic!("These matrices cannot be multiplied")
//         }
//     }
// }

impl <T:TensorTrait<T>> Mul<T> for Tensor<T> 
where
    T: Mul<Output = T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut temp: Vec<T> = Vec::new();

        for x in 0..self.inner.len()-1 {
            let a = self.inner[x] * rhs;

            temp.push(a);

        };
        
        Self {
            inner: temp.into_boxed_slice(),
            shape: self.shape,
            // strides: self.strides,
        }
    }
}

//(Box<[T]>, Box<[usize]>)
pub fn matmul<T>(x1: Box<[T]>, x1_shape: Box<[usize]>, x2: Box<[T]>, x2_shape: Box<[usize]>) -> () {

    if x1_shape == x2_shape {
        if x1_shape.len() > 2 && x2_shape.len() > 2 {
            let x1_shape_max = x1_shape.iter().fold(1, |acc, x| acc * x);
            let x2_shape_max = x2_shape.iter().fold(1, |acc, x| acc * x);
    
            let x1_layers = (0..x1.len()-1).step_by(x1.len()/x1_shape[0]).map()
    
        } else {
            return 
        }

    } else {
        panic!("matrices are not the same shape");
    }
}


impl <T: TensorTrait<T>> Mul<Tensor<T>> for Tensor<T> 
where
    T: Mul<Output = T>
{
    type Output = Self;
    fn mul(self, rhs: Tensor<T>) -> Tensor<T> {

        let t = matmul(self.inner, self.shape, rhs.inner, rhs.shape);

        // Self {
        //     inner: t.0,
        //     shape: t.1
        // }
        self
        

    }
}


// impl <T:TensorTrait<T>> Mul<Tensor<T>> for Tensor<T>
// where
//     T: Mul<Output = T>
// {
//     type output=Self
// }

impl <T: TensorTrait<T>> Add for Tensor<T>
where T: Add<Output = T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        

        if self.shape == rhs.shape {
            let mut t: Vec<T> = Vec::new();

            for x in 0..self.inner.len()-1 {
                t.push(self.inner[x] + rhs.inner[x])
            }

            Self {
                inner   : t.into_boxed_slice(),
                shape   : self.shape,
                // strides : self.strides,
            }
        } else {
            panic!("These tensors do not have the same shape");
        }
        
    }
}