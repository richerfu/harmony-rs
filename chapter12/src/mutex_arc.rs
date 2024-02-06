use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

#[derive(Debug)]
struct User<'a> {
    pub name: &'a str,
}

pub fn arc_test() {
    let a = Arc::new(AtomicI32::new(0));
    let handle_a = a.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..10000 {
            handle_a.fetch_add(1,Ordering::Relaxed);
        }
    });
    let handle_b = a.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..10000 {
            handle_b.fetch_add(1,Ordering::Relaxed);
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{:?}",a);
}

pub fn mutex_test() {
    let a = Arc::new(Mutex::new(-1));

    let mut handle = Vec::new();

    for i in 0..5 {
        let code = a.clone();
        handle.push(thread::spawn(move || {
            let mut data = code.lock().unwrap();
            (*data) = i;
            println!("{:?}", &data);
        }))
    }
    for h in handle {
        h.join().unwrap();
    }
}