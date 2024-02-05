use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct User<'a> {
    pub name: &'a str,
}

pub fn arc_test() {
    // let a = User { name: "hello" };
    // let b = a;
    //
    // println!("{:?},{:?}", a, b);

    let mut a = Rc::new(User { name: "hello" });

    let b = a.clone();

    println!("{:?},{:?}", a, b);
}

pub fn mutex_test() {
    let a = Arc::new(Mutex::new(User { name: "rust" }));

    let mut handle = Vec::new();

    for i in 0..5 {
        let code = a.clone();
        handle.push(thread::spawn(move || {
            let mut data = code.lock().unwrap();
            let c =  format!("rust {:?}", thread::current().id()).clone();
            data.name = "test";
            println!("{:?}", &data);
        }))
    }
    for h in handle {
        h.join().unwrap();
    }
}