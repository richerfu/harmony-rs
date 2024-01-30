use std::fmt::{Debug, Formatter};

#[repr(C)]
pub struct User {
    pub name: String,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User.name: {}", self.name)
    }
}
