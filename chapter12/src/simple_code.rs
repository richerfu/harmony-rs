use std::thread;

pub fn simple_thread() {
    let mut t = Vec::new();
    for i in 0..5 {
        t.push(thread::spawn(move || {
            println!("{i}");
        }))
    }
    for i in t {
        i.join().unwrap();
    }
}