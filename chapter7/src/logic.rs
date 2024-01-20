pub fn run_loop() {
    let mut i = 1;
    loop {
        i = i + 1;
        if i > 5 {
            break;
        }
    }
}

pub fn run_for() {
    for i in 1..10 {
        println!("{i}")
    }
}

pub fn run_three() {
    let a = if true { 1 } else { 0 };
}

pub fn run_while() {
    let mut i = 1;
    while i < 10 {
        i = i + 1;
        if i > 5 {
            break;
        }
    }
}

pub fn run_if() {
    let mut i = 1;
    if (true) {
        println!("{i}")
    } else if false {
        println!("hello")
    } else {
        println!("world")
    }
}