pub mod tensor;



fn main() {

    let data = vec![
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
            vec![24,25,26],
        ],

    ];

    let a = new_from_matrix!(i32, data.to_owned(), [3,3,3]);
    let b = new_from_matrix!(i32, data.to_owned(), [3,3,3]);

    let c = a * b;



}
