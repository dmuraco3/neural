pub mod tensor;

fn main() {
    let t = tensor![
        [
            [0,0,0],
            [0,0,0],
            [0,0,0]
        ],
        [
            [0,0,0],
            [0,0,0],
            [0,0,0]
        ],
        
    ];

    let scalar = tensor![0];

    println!("{:?}", scalar.shape);
}
