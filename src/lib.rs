mod tensor;

#[allow(dead_code)]
const TEST_RUNS : u32 = 10000;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{new_from_matrix, TEST_RUNS};

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

        println!("total time: {:#?}, average: {:#?} (2 * 3 matrix multiplication, {} iterations) ", duration, duration / TEST_RUNS, TEST_RUNS);

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
                vec![3,4],
            ],
            vec![
                vec![5,6],
                vec![7,8],
                vec![9,10]
            ],
            vec![
                vec![11,12],
                vec![13,14],
                vec![15,16]
            ]

        ];

        let x1_tensor = new_from_matrix!(i32, x1, [3,3,3]);
        let x2_tensor = new_from_matrix!(i32, x2, [3,3,2]);

        let result = x1_tensor * x2_tensor;


    }
}