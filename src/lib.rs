pub mod tensor;
pub mod nn;

#[allow(dead_code)]
const TEST_RUNS : usize = 1000000;

#[cfg(test)]
mod tests {
    use std::time::{Instant, Duration};

    use crate::{new_from_matrix, TEST_RUNS, tensor::Tensor};

    #[test]
    fn test_2d_multiplication() {
        let x1 = vec![
            vec![0,1,2],
            vec![3,4,5],
        ];

        let x2 = vec![
            vec![0,1],
            vec![2,3],
            vec![4,5]
        ];

        let x1_tensor = new_from_matrix!(i32, x1, [2,3]);
        let x2_tensor = new_from_matrix!(i32, x2, [3,2]);

        let result = x1_tensor.clone() * x2_tensor.clone();

        let correct_result = new_from_matrix!(
            i32,
            vec![
                vec![10,13],
                vec![28,40]
            ],
            [2,2]
        );

        let start = Instant::now();
        for _ in 0..TEST_RUNS {
            let _ = x1_tensor.clone() * x2_tensor.clone();
        }
        let duration  = start.elapsed();

        println!("total time: {:#?}, average: {:#?} (2 * 3 matrix multiplication, {} iterations) ", duration, duration / TEST_RUNS as u32, TEST_RUNS);

        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_3d_multiplication() {
        let x1 = vec![
            vec![
                vec![0,1,2],
                vec![3,4,5],
                vec![6,7,8],
            ],
            vec![
                vec![9,10,11],
                vec![12,13,14],
                vec![15,16,17],
            ],
            vec![
                vec![18,19,20],
                vec![21,22,23],
                vec![24,25,26]
            ]
        ];

        let x2 = vec![
            vec![
                vec![0,1],
                vec![2,3],
                vec![4,5],
            ],
            vec![
                vec![6,7],
                vec![8,9],
                vec![10,11]
            ],
            vec![
                vec![12,13],
                vec![14,15],
                vec![16,17]
            ]

        ];

        let x1_tensor = new_from_matrix!(i32, x1, [3,3,3]);
        let x2_tensor = new_from_matrix!(i32, x2, [3,3,2]);

        let result = x1_tensor.clone() * x2_tensor.clone();

        let correct_result = Tensor::new([10, 13, 28, 40, 46, 67, 244, 274, 316, 355, 388, 436, 802, 859, 928, 994, 1054, 1129].to_vec(), [3,3,2].to_vec());

        // benchmark test here because benchmarks are only in nightly smh
        {

            let mut times = Vec::with_capacity(TEST_RUNS);
    
            for _ in 0..TEST_RUNS {
                let start = Instant::now();
    
                let _ = x1_tensor.clone() * x2_tensor.clone();
    
                times.push(start.elapsed());
            }
    
            let mean : u32 = times.iter().sum::<Duration>().subsec_nanos() / TEST_RUNS as u32;
    
            let std_dev : u32 = times.iter().map(|x| (x.subsec_nanos() - mean).pow(2)).sum::<u32>() / TEST_RUNS as u32;
    
            println!("{:#?} ± {:#?} per loop", Duration::from_nanos(mean as u64), Duration::from_nanos(std_dev as u64) );

        }


        assert_eq!(result, correct_result);

    }
}
