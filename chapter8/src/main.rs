mod owner;
mod copy_clone;

fn main() {
    // owner::run_owner_test_1();
    let ref a = 1;
    let ref b = 2;
    copy_clone::get_bigger(a,b);
}
