use std::borrow::BorrowMut;
use std::iter::Sum;
use std::ops::AddAssign;
use std::ops::Mul;

use crate::tensor::Tensor;
use crate::tensor::TensorTrait;
use crate::tensor::utils::alloc_box_buffer;

#[inline(never)]
pub fn matmul<T: TensorTrait<T, T=T>>(x1:Tensor<T>, x2: Tensor<T>) -> Tensor<T>
where
    T: Sum<T> + Mul<T, Output = T> + AddAssign<T>,
    for <'a> &'a T: Mul<&'a T, Output = T>,
{
    let mut x2 = x2.clone();

    x2.borrow_mut().transpose();


    let mut temp = alloc_box_buffer::<T>(x1.inner.len());

    let mut index = 0;

    for row in x1.inner.chunks_exact(x1.shape[1]) {
        for col in x2.inner.chunks_exact(x1.shape[1]) {
            let mut sum = T::default();
            // let dot = row.iter().zip(col).map(|(x,y)| x*y).sum::<T>();
            row.chunks(16).zip(col.chunks(16)).for_each(|(x,y)| {
                sum += x.iter().zip(y.iter()).map(|(x,y)| x*y).sum::<T>();
            });
            // for (x,y) in row.iter().zip(col.iter()) {
            //     sum += x*y;
            // }
            temp[index] = sum;
            index+=1
        }
    }

    return Tensor { 
        inner: temp,
        shape: x1.shape.clone(),
    }
    


}

/// Multiplies a Tensor by a scalar value of type T
#[inline(never)]
pub fn scalar_matmul<T: TensorTrait<T>>(x1:&Tensor<T>, x2: T) -> Tensor<T>
where
{

    let mut new_data = alloc_box_buffer::<T>(x1.inner.len());

    return Tensor { 
        inner: new_data,
        shape: x1.shape.clone(),
    }
    


}