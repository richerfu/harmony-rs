mod example;

use std::ops::Add;

struct User {
    pub id: u32,
    pub name: String,
}

impl User {
    fn new() -> User {
        User {
            id: 1,
            name: String::from("bob"),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

trait Animal {
    fn log() {}
}

trait BigCat: Animal {
    fn run() {}
}

trait Color {
    fn color() {}
}

trait Toms: BigCat + Color {
    fn speak() {}
}

type Cat = ();
type Dog = ();

// impl Animal for Cat {
//     fn log() {
//         println!("cat")
//     }
// }

impl Animal for Dog {
    fn log() {
        println!("dog")
    }
}

pub struct Tom();

impl Animal for Tom {
    fn log() {
        todo!()
    }
}

impl BigCat for Tom {
    fn run() {
        todo!()
    }
}

fn run<T: std::ops::Add<Output = T>>(a: T,b: T) -> T {
    a + b
}

fn main() {
    // let u = User::new();
    // let id = u.get_id();
    // println!("{id}");

    example::test_fn();
}
