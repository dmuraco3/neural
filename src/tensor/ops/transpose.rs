use crate::tensor::{Tensor, TensorTrait, utils::alloc_box_buffer};



impl <T:TensorTrait<T>> Tensor<T> 
{
    #[inline(never)]
    pub fn transpose(&mut self) {
        
        
        if self.shape.len() == 2 {
            
            let mut temp = alloc_box_buffer::<T>(self.inner.len());

            let cols = self.shape[1];
            let rows = self.shape[0];

            for x in 0..cols {
                for y in (0..self.inner.len()-1).step_by(cols) {
                    temp[(x * rows) + (y/cols) ] = self.inner[x + y];
                }
            }
            self.inner = temp;
            self.shape.reverse();




        } else {
            panic!("only 2*2 transposes supported at the moment")
        }

    }
}

