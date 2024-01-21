fn run_test(flag: &bool) {
    println!("{:?}", flag);
}

pub fn run_copy() {
    let name = false;

    let a = &name;
    run_test(a);
    println!("{:?}", a);
}

fn print_user_id(mut u: User) {
    u.id = 2;
    println!("{:?}", u.id);
}

#[derive(Copy, Clone)]
struct User {
    id: u32,
}

pub fn run_copy_1() {
    let mut bob = User { id: 1 };
    print_user_id(bob);
    println!("{:?}", bob.id);
}

pub fn run_copy_2() {
    let a = 5;
    let b = &5;
    println!("{}", *b);
}

fn print_user_id_1(u: &Class) {
    println!("{:?}", u.id);
}

struct Class {
    id: u32,
}

pub fn run_copy_3() {
    let class_1 = Class { id: 1 };
    let ref_class = &class_1;
    print_user_id_1(ref_class);
    println!("{:?}", (*ref_class).id);
}

pub fn get_bigger<'a>(a: &'a u32, b: &'a u32) -> &'a u32 {
    if (*a) > (*b) {
        a
    } else {
        b
    }
}

