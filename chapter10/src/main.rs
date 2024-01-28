macro_rules! my_vec {
    ($type: literal) => {
        Vec::with_capacity($type)
    };
    ($($type: expr),*) => {
        vec!($($type),*)
    };
}

fn main() {
    let a:Vec<i64> = my_vec!(10);
    let b = my_vec!(1,2,3);
}
