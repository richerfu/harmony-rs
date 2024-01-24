use std::ops::Add;

trait Animal {
    fn log() {}
}

trait Cat: Animal {
    fn jump() {}
}

trait Mouse: Animal {
    fn run() {}
}

struct Tom {
    name: String,
}

struct Jerry {
    name: String,
}

impl Add<Jerry> for Tom {
    type Output = String;
    fn add(self, rhs: Jerry) -> Self::Output {
        format!("{} and {}", self.name, rhs.name)
    }
}

pub fn test_fn() {
    let tom = Tom { name: String::from("tom") };
    let jerry = Jerry { name: String::from("jerry") };
    let result = tom + jerry;
    println!("{}", result);
}