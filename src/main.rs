pub mod tensor;



fn main() {

    let data = vec![
        0,0,0
    ];

    let t = new_from_matrix!(i32, data);
    println!("{:?}", t);

}
