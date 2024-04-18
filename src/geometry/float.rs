use std::f64::EPSILON;
use std::f64::{MAX, MIN};

pub fn nearly_equal(a: f64, b: f64) -> bool {
    let diff = (a - b).abs();
    let norm = (a.abs() + b.abs()).min(MAX);

    diff < MIN.max(EPSILON * norm)
}
