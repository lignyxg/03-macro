

#[macro_export]
macro_rules! my_vec {
    () => {Vec::new()};
    ($elem:expr; $n:expr) =>{
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        {
            // let mut temp_vec = Vec::new();
            // $(
            // temp_vec.push($x);
            // )*
            // temp_vec
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}

fn main() {
    let my_vec = my_vec![1,2,3,4];
    println!("{:?}", my_vec);
}

