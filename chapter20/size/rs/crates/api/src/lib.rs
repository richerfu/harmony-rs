mod register;
mod value;

use backend::api;

#[api]
pub fn add(left: f64, right: f64) -> f64 {
    left + right
}
