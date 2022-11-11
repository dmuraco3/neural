use std::time::Instant;

use rustgrad::{new_from_matrix, tensor::ops::matmul::matmul};

fn main() {
    
    const SIZE: usize = 512;

    let test: [[f32;SIZE];SIZE] = [[20.0231;SIZE];SIZE];



    let x1 = new_from_matrix!(f32, test.clone(), [SIZE,SIZE]);

    let x2 = new_from_matrix!(f32, test.clone(), [SIZE,SIZE]);

    let start = Instant::now();
    let _ = matmul(x1.clone(), x2.clone());

    println!("{:#?}", start.elapsed());

    println!("{:?}", std::thread::available_parallelism().unwrap().get());


}
