pub fn add(a: u32,b: u32) -> u32 {
    a + b
}

fn main() {
    println!("Hello, Rust!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2,2), 4);
    }
}
