// reference: https://stackoverflow.com/a/32334103/2212464

use std::f64::{MAX, MIN_POSITIVE};

const EPSILON: f64 = 1e-8;
const ABS_TH: f64 = MIN_POSITIVE;

pub fn nearly_equal(a: f64, b: f64) -> bool {
    if a == b {
        true
    } else if nearly_zero(a) {
        nearly_zero(b)
    } else if nearly_zero(b) {
        nearly_zero(a)
    } else {
        let diff = (a - b).abs();
        let norm = MAX.min(a.abs() + b.abs());

        diff < ABS_TH.max(EPSILON * norm)
    }
}

pub fn nearly_zero(a: f64) -> bool {
    if a == 0. {
        true
    } else {
        a.abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(nearly_equal(1., 1.));
    }

    #[test]
    fn test_2() {
        assert!(!nearly_equal(1., 0.));
    }

    #[test]
    fn test_3() {
        assert!(!nearly_equal(1., 0.999999));
    }
}
