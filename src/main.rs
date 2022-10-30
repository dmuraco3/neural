use std::time::Instant;

use rustgrad::new_from_matrix;

fn main() {
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

        let start = Instant::now();

        for _ in 0..10000 {
            let _ = x1_tensor.clone() * x2_tensor.clone();
        }

        let duration = start.elapsed();

        println!("took {:#?}, average of {:#?} over {} runs", duration, duration /10000, 10000);

}
