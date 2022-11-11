use std::iter::Sum;
use std::mem;
use std::ops::AddAssign;
use std::ops::Mul;

use crate::tensor::Tensor;
use crate::tensor::TensorTrait;




#[inline(never)]
pub fn matmul<T: TensorTrait<T, T = T>>(x1:Tensor<T>, x2: Tensor<T>) -> Tensor<T>
where
    T: Mul<T, Output = T>  +  AddAssign<T> + Sum<T>,
    for<'a> &'a T: Mul<&'a T, Output = T>
{
    
    let mut x2 = x2;
    x2.transpose();
    
    let num_rows = x1.shape[0];

    let chunksize = num_rows / (num_rows / (mem::size_of::<T>() * 8));
    
    let t = x1.inner.chunks(num_rows).map(|row| 
        x2.inner.chunks(num_rows).map(move |col| 
            col.chunks(chunksize).zip(row.chunks(chunksize)).map(
                |(x,y)|
                    x.iter().zip(y).map(|(a, b)| a*b).sum::<T>()
            ).sum::<T>()
            
        ).collect::<Vec<T>>()
    ).flatten().collect::<Vec<T>>();

    Tensor {
        inner: t.into_boxed_slice(),
        shape: x1.shape.clone(),
    }

}