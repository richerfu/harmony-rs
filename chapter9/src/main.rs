struct User {
    pub id: u32,
    pub name: String,
}

impl User {
    fn new () -> User {
        User {
            id: 1,
            name: String::from("bob")
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

fn main() {
    let u = User::new();
    let id = u.get_id();
    println!("{id}");
}
