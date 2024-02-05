mod simple_code;
mod closure;
mod mutex_arc;

fn main() {
    simple_code::simple_thread();
    closure::closure_object_test();
    mutex_arc::arc_test();
    mutex_arc::mutex_test();
}
