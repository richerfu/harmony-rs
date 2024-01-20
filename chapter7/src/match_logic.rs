pub struct User {
    pub id: u32,
    pub name: String,
}

enum MatchEnum {
    Start,
    Login,
    User(User),
}

pub fn match_logic() {
    let a: MatchEnum = MatchEnum::User(User { id: 1, name: String::from("hello") });

    let status = match a {
        MatchEnum::Login => true,
        MatchEnum::Start => false,
        MatchEnum::User(user) => {
            println!("{}", user.id);
        }
    };
}
