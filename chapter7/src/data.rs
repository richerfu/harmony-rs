const a: (i32, &str, bool) = (1, "hello", false);

struct ComplexData {
    id: i64,
    name: String,
}

#[derive(PartialEq)]
pub enum ComplexEnum {
    Logout,
    Detail,
    Start = 255,
    Login
}

pub struct User {
    pub id: u32,
    pub name: String
}

pub enum StructEnum {
    User(User)
}

pub fn test_enum () {
    let bob = StructEnum::User(User { id: 1, name: String::from("Bob") });
    let lucy = StructEnum::User(User { id: 2, name: String::from("Lucy") });
}