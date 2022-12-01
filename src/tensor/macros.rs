#[macro_export]
macro_rules! tensor {
    (
        //10
        $([$([$([$([$([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*

    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*],)*],)*],)*],)*];

            let dims = [
                t[0][0][0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0].len(),
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                10
            ]

        }
    }};
    (
        //9
        $([$([$([$([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*

    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*],)*],)*],)*];
            
            let dims = [
                t[0][0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0].len(),
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                9
            ]

        }
    }};
    (
        //8
        $([$([$([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*],)*],)*];

            let dims = [
                t[0][0][0][0][0][0][0].len(),
                t[0][0][0][0][0][0].len(),
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                8
            ]
        }
    }};
    (
        //7
        $([$([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*],)*];

            let dims = [
                t[0][0][0][0][0][0].len(),
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                7
            ]

        }
    }};
    (
        //6
        $([$([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*],)*];
            let dims = [
                t[0][0][0][0][0].len(),
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                6
            ]
        }
    }};
    (
        //5
        $([$([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*]) , + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*],)*];
            let dims = [
                t[0][0][0][0].len(),
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                5
            ]
        }
    }};
    (
        //4
        $([$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]), + $(,)*
    ) => {{
        {
            let t = vec![$(vec![$(vec![$(vec![$($x,)*],)*],)*],)*];
            let dims = [
                t[0][0][0].len(),
                t[0][0].len(),
                t[0].len(),
                4
            ]
        }
    }};
    (
        //3
        $d_type: ty,
        [$([$([$($x:expr),* $(,)*]), + $(,)*]), + $(,)*]
    ) => {{
        {
            let t = [$([$([$($x,)*],)*],)*];

            let dims = [
                t[0][0].len(),
                t[0].len(),
                3
            ];

            let dim_prod = dims.iter().fold(1, |sum, val| sum * val);


            let p = t.iter().flat_map(|x| x.iter()).flat_map(|x| x.iter()).cloned().collect::<Vec<$d_type>>();



            "bruh"

        }
    }};
    (
        //2
        $dtype: ty,
        [$([$($x:expr),* $(,)*]),+ $(,)*]
    ) => {{
        {
            let t = vec![$([$($x,)*],)*];

            let dims = [
                t[0].len(),
                2
            ];
            
            let dim_prod = dims.iter().fold(1, |sum, val| sum * val);



        }
    }};
    (
        //1
        $dtype: ty,
        [$($x:expr),* $(,)*]
    ) => {{
        {
            let t = vec![$($x,)*];
            let dims = [t.len()];
        }

    }};
    
}
#[macro_export]
macro_rules! new_tensor {
    ($dtype: ty, $data: expr, $shape: expr) => {{

        use $crate::tensor::utils::RecursivelyFlattenIterator;
        
        use $crate::tensor::Tensor;

        // simple shape check
    
        let data = $data.into_iter().recursively_flatten::<_, $dtype>().collect::<Vec<$dtype>>();

        let data = data.into_boxed_slice();
        if $shape.iter().fold(1, |acc, x| acc * x) == data.len() {
            Tensor::new(
                data,
                Box::new($shape)
            )
        } else {
            panic!("Shape doesn't match array")
        }

        
    }};
}