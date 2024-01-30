use custom_macro::MyDebug;

mod custom;
mod inner;

#[derive(MyDebug)]
struct Name {
    id: i64,
    name: String,
}

#[derive(MyDebug)]
struct TestName {
    desc: String,
}

fn main() {
    let user = custom::User {
        name: String::from("rust"),
    };
    let inner_user = inner::BuiltUser {
        name: String::from("rust"),
    };
    // println!("{:?}", user);
    println!("{:?}", inner_user);

    let n = Name {
        id: 1,
        name: String::from("hello"),
    };
    let d = TestName {
        desc: String::from("this is a desc"),
    };
    println!("{:?}", d);
    println!("{:?}", n);
}
