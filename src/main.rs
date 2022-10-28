use std::time::Instant;

pub mod tensor;



fn main() {

    let x1 = vec![
        vec![0,1,2],
        vec![3,4,5],
        vec![6,7,8]

    ];

    let x2 = vec![
        vec![0,1,2],
        vec![3,4,5],
        vec![6,7,8]
    ];

    let a = new_from_matrix!(i32, x1, [3,3]);
    let b = new_from_matrix!(i32, x2, [3,3]);


    let start = Instant::now();
    for x in 0..1000 {
        let _ = a.clone() * b.clone();
    }
    let duration = start.elapsed();

    println!("1000 iterations of multiplying 3 * 3 matrices took {:#?} with average of {:#?} per iteration", duration, duration / 1000);

    drop(a);
    drop(b);


    let x1 = vec![
        vec![
            vec![0,1,2],
            vec![3,4,5],
            vec![6,7,8]
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
        ],
    ];

    let a = new_from_matrix!(i32, x1, [3,3,3]);

    // shape[1] * shape[2] = shape[ index.len().. ].to_vec().iter().fold(1 |acc , x| acc * x);
    //
    // a[0] a.inner[0..9]    increments by shape[1] * shape[2]
    // a[1] a.inner[9..18]   [index * (shape[1] * shape[2]) .. shape[1] * shape[2]] // checks out 
    // a[2] a.inner[18..27]  [index * (shape[1] * shape[2]) .. shape[1] * shape[2]] // checks out 
    //
    // shape[2] = shape[ index.len().. ].to_vec().iter().fold(1 |acc , x| acc * x)
    //
    // a[0][0] a.inner[0..3] // increments by shape[2]
    // a[0][1] a.inner[3..6] [ ( index[0] + index[1] ) * shape[2]]
    // a[0][2] a.inner[6..9]
    //
    // a[1][0] a.inner[9..12]
    // a[1][1] a.inner[12..15]
    // a[1][2] a.inner[15..18]
    //
    // a[2][0] a.inner[18..21]
    // a[2][1] a.inner[21..24]
    // a[2][2] a.inner[24..27]

    let x2 = vec![
        vec![
            vec![0,1,2],
            vec![3,4,5],
        ],
        vec![
            vec![6,7,8],
            vec![9,10,11],
        ],
        vec![
            vec![12,13,14],
            vec![15,16,17],
        ],
    ];

    let b = new_from_matrix!(i32, x2, [3,2,3]);

    // b[0] b.inner[0..6]   increments by shape[1] * shape[2]
    // b[1] b.inner[6..12]  [index * (shape[1] * shape[2])..index * (shape[1] * shape[2]) * shape[1] ]
    // b[2] b.inner[12..18]
    //
    // b[0][0] b.inner[0..3]    increments by shape[2]
    // b[0][1] b.inner[3..6]    [ ( index[0] + index[0] + index[1] ) * shape[2] .. ( index[0] + index[1] + index[0] ) * shape[2] + shape[2] ] // checks out
    //
    // b[1][0] b.inner[6..9]    [ ( index[0] + index[0] + index[1] ) * shape[2] .. ( index[0] + index[1] + index[0] ) * shape[2] + shape[2] ] // checks out
    // b[1][1] b.inner[9..12]
    //
    // b[2][0] b.inner[12..15]
    // b[2][1] b.inner[15..18]
    //
    // b[0][0][0] b.inner[0..1] 
    // b[0][0][1] b.inner[1..2]
    // b[0][0][2] b.inner[2..3]
    //
    // b[0][1][0] b.inner[3..4]
    // b[0][1][1] b.inner[4..5]
    // b[0][1][2] b.inner[5..6]
    //
    // [1][0][0] b.inner[6..7]
    // [1][0][1] b.inner[7..8]
    // [1][0][2] b.inner[8..9]
    //
    // [1][1][0] b.inner[9..10]  [ ( index[0] + index[0] + index[1] ) * shape[2] .. ( index[0] + index[1] + index[0] ) * shape[2] +  ]
    // [1][1][1] b.inner[10..11] 
    // [1][1][2] b.inner[11..12]




    let x3 = vec![
        vec![
            vec![
                vec![0,1,2],
                vec![3,4,5],
                vec![6,7,8],
                vec![9,10,11],
            ],
            vec![
                vec![12,13,14],
                vec![15,16,17],
                vec![18,19,20],
                vec![21,22,23],
            ]
        ],
        vec![
            vec![
                vec![24,25,26],
                vec![27,28,29],
                vec![30,31,32],
                vec![33,34,35],
            ],
            vec![
                vec![36,37,38],
                vec![39,40,41],
                vec![42,43,44],
                vec![45,46,47],
            ]
        ],
        vec![
            vec![
                vec![48,49,50],
                vec![51,52,53],
                vec![54,55,56],
                vec![57,58,59],
            ],
            vec![
                vec![60,61,62],
                vec![63,64,65],
                vec![66,67,68],
                vec![69,70,71],
            ]
        ]
    ];

    let c =  new_from_matrix!(i32, x3, [3,2,4,3]);

    // c[0] b.inner[0..24]   [ shape[1] * shape[2] * shape[3] ]
    // c[1] b.inner[24..48]  [ index[0] * (shape[1] * shape[2] * shape[3]) .. index[0] * (shape[1] * shape[2] * shape[3]) + (shape[1] * shape[2] * shape[3]) ]
    // c[2] b.inner[48..72]  [ ]



    let _ = a * b;
}
