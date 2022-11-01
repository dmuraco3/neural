use std::{fmt::{Debug}, ops::{Mul, Add}, iter::Sum};

use num::Zero;

pub mod utils;

pub mod macros;

pub trait TensorTrait<T>: Debug +  Copy + Zero  + 'static {}

impl <T> TensorTrait<T> for i32 {}

impl <T> TensorTrait<T> for f32 {}

impl <T> TensorTrait<T> for f64 {}



#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct Tensor<T: TensorTrait<T>>
where
    T : Copy
{
    pub inner      : Vec<T>,

    pub shape      : Vec<usize>,

    // strides: Box<[usize]>,
}

#[allow(dead_code)]
impl <T: TensorTrait<T>> Tensor<T> {
    pub fn new(data: Vec<T>, dims: Vec<usize>) -> Self {
        Self {
            inner  : data,
            shape  : dims,
        }
    }

    pub fn len(self) -> usize {
        return self.inner.len()
    }


    // indexing in this library works like array accesses so that [x,y,z,...] are reversed [...,z,y,x]
    fn index(self, coordinates: Vec<usize>)  -> T {
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


impl <T:TensorTrait<T>> Mul<T> for Tensor<T> 
where
    T: Mul<Output = T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut temp: Vec<T> = Vec::with_capacity(self.inner.len());

        for x in 0..self.inner.len()-1 {
            let a = self.inner[x] * rhs;

            temp.push(a);

        };
        
        Self {
            inner: temp,
            shape: self.shape,
            // strides: self.strides,
        }
    }
}

// returns vector all the time so we can get slices from this 
pub fn index<T>(x: Vec<T>, x_shape: Vec<usize>, indices: Vec<usize>) -> Result<(Vec<T>, Vec<usize>), String>
where
    T: Copy
{
    if indices.len() == x_shape.len() {
        let mut sum = 0;
        for x in 0..x_shape.len()-1 {
            let y = x_shape[x+1..].iter().fold(1, |acc, x| acc * x);
            sum += y * indices[x]
        }
        sum += indices[indices.len()-1];
        return Ok((
            Vec::from([x[sum]]),
            Vec::new()
        ))
    } 
    else if indices.len() < x_shape.len() {
        
        //  a = ( indices * indices.len()..1 )
        //  b = ( inner_prod( shape[indices.len()..] ) )
        //
        //  returns a[ a * b ..  a * b + b]

        let indice_calc = indices.iter().zip((1..=indices.len()).rev()).map(|(indice, reverse_index)| indice * reverse_index).fold(0,|acc,x| acc+ x);
        let shape_calc = x_shape[indices.len()..].iter().fold(1, |acc, x| acc * x);
        let t = x[
            indice_calc * shape_calc
            ..
            indice_calc * shape_calc + shape_calc
        ].to_vec();
        
        return Ok((
            t,
            x_shape[indices.len()..].to_vec(),
        ))
    }
    else {
        return Err("indices longer than shape".into());
    }
}

// dot product
pub fn dotprod<T>(x1: Vec<T>, x1_shape: Vec<usize>, x2: Vec<T>, x2_shape: Vec<usize>) -> (Vec<T>, Vec<usize>)
where
    T: Mul<Output = T> + Copy + Sum
{
    if x1_shape.len() == 0 && x2_shape.len() == 0 {
        return (Vec::from([x1[0] * x2[0]]), Vec::new())
    }
    else if x1_shape.len() == 1 && x2_shape.len() == 1 {
        return (
            Vec::from([x1.iter().cloned().zip(x2.iter().cloned()).map(|(x1,x2)| x1 * x2).sum()]),
            Vec::new()
        )
    }
    else if x1_shape.len() == 2 && x2_shape.len() == 2 {
        return matmul(x1, x1_shape, x2, x2_shape)
    }
    else {
        panic!("can't perform this operation")
    }

}

//(Box<[T]>, Box<[usize]>)
pub fn matmul<T>(x1: Vec<T>, x1_shape: Vec<usize>, x2: Vec<T>, x2_shape: Vec<usize>) -> (Vec<T>, Vec<usize>)
where
    T: Mul<Output = T> + Copy + Sum
{
    let shape_size = (x1_shape.len(), x2_shape.len());
    if shape_size == (0,0) {
        return (
            Vec::from([x1[0] * x2[0]]),
            Vec::new()
        )
    }
    // 1d matrices can't be matrix multiplied
    else if shape_size == (2,2) {
        if x1_shape[1] == x2_shape[0] {
            
            let mut cols: Vec<Vec<T>> = Vec::with_capacity(x2_shape[0]);


            // O(x2_shape[1]^2)
            // gets cols here so using less operations in row iteration
            for col_index in 0..x2_shape[1] {
                let mut col: Vec<T> = Vec::with_capacity(x2_shape[0]);
                for row_index in (0..x2.len()-1).step_by(x2_shape[1]) {
                    col.push(x2[col_index + row_index])
                }
                cols.push(col)
            }


            let mut new_matrix: Vec<T> = Vec::with_capacity(x1_shape[0] * x2_shape[1]);
            for row_index in (0..x1.len()-1).step_by(x1_shape[1]) {
                let row = x1[row_index..row_index + x1_shape[1]].to_vec();
                for col in cols.iter().cloned() {
                    let temp = dotprod(row.to_owned(), Vec::from([x1_shape[1]]), col, Vec::from([x2_shape[0]])).0[0];
                    new_matrix.push(temp)
                }
            }

            return (
                new_matrix,
                Vec::from([x1_shape[0], x2_shape[1]])
            )
        } else {
            panic!("row size does not match column size");
        }
    }
    else if shape_size > (2,2) {

        let result = (0..x1_shape[0]).map(|x| {
            let x1_sub = index(x1.clone(), x1_shape.clone(), Vec::from([x])).unwrap();
            let x2_sub = index(x2.clone(), x2_shape.clone(), Vec::from([x])).unwrap();
            let result = dotprod(x1_sub.0, x1_sub.1, x2_sub.0, x2_sub.1);
            result.0
        }).clone().flatten().collect::<Vec<T>>();

        // let shape = [x1_shape[0..x1_shape.len()-2].borrow().concat(), x2_shape[x2_shape.len()-1]]

        let mut shape = x1_shape[0..x1_shape.len()-1].to_vec();

        shape.extend([x2_shape[x2_shape.len()-1]].iter());

        return (
            result,
            shape,
        )
    }
    else {
        panic!("These matrices can't be multiplied")
    }
    
}


impl <T: TensorTrait<T>> Mul<Tensor<T>> for Tensor<T> 
where
    T: Mul<Output = T> + Sum
{
    type Output = Self;
    fn mul(self, rhs: Tensor<T>) -> Tensor<T> {

        let t = matmul(self.inner, self.shape, rhs.inner, rhs.shape);

        Self {
            inner: t.0,
            shape: t.1
        }
        

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
                inner   : t,
                shape   : self.shape,
                // strides : self.strides,
            }
        } else {
            panic!("These tensors do not have the same shape");
        }
        
    }
}