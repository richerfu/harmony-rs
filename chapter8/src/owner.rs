fn getNameLength(str: bool) -> () { println!("{str}") }

pub fn run_owner_test() {
    let name = false;
    getNameLength(name);

    println!("value is {name}");
}

fn get_name(str: String) -> String {
    println!("{:?}", str);
    return str;
}

pub fn run_owner_test_1() {
    let a = String::from("rust");

    let b = get_name(a);

    println!("{:?}", b);
}