mod simple_module;
mod folder_module;

use simple_module::test_fn;
use folder_module::util;

fn main() {
    test_fn();
    util::get_file();
}
