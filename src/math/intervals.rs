use std::cmp::Ordering;
use std::fmt::Display;

pub enum Bound {
    Open(f64),
    Closed(f64),
    NegativeInfinity,
    PositiveInfinity,
}

impl Bound {
    pub fn unwrap(&self) -> f64 {
        match self {
            Self::Closed(k) | Self::Open(k) => *k,
            Self::NegativeInfinity => f64::NEG_INFINITY,
            Self::PositiveInfinity => f64::INFINITY,
        }
    }
}

impl PartialEq for Bound {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bound::NegativeInfinity, _)
            | (Bound::PositiveInfinity, _)
            | (_, Bound::NegativeInfinity)
            | (_, Bound::PositiveInfinity) => false,
            (x, y) => x.unwrap() == y.unwrap(),
        }
    }
}

impl PartialOrd for Bound {
    fn lt(&self, other: &Self) -> bool {
        self.unwrap() < other.unwrap()
    }

    fn le(&self, other: &Self) -> bool {
        self.unwrap() <= other.unwrap()
    }

    fn gt(&self, other: &Self) -> bool {
        self.unwrap() > other.unwrap()
    }

    fn ge(&self, other: &Self) -> bool {
        self.unwrap() >= other.unwrap()
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self > other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

pub struct Interval {
    inf: Bound,
    sup: Bound,
}

const EMPTY: Interval = Interval {
    inf: Bound::Open(0.),
    sup: Bound::Open(0.),
};

impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}

impl Interval {
    pub fn new(k1: Bound, k2: Bound) -> Self {
        assert!(k1 <= k2);
        Interval { inf: k1, sup: k2 }
    }

    pub fn singleton(k: f64) -> Interval {
        Interval {
            inf: Bound::Closed(k),
            sup: Bound::Closed(k),
        }
    }

    pub fn empty() -> Self {
        EMPTY
    }

    pub fn is_singleton(&self) -> bool {
        match (&self.inf, &self.sup) {
            (Bound::Closed(k1), Bound::Closed(k2)) => k1 == k2,
            _ => false,
        }
    }
    pub fn get_singleton(&self) -> Option<f64> {
        match (&self.inf, &self.sup) {
            (Bound::Closed(k1), Bound::Closed(k2)) => {
                if k1 == k2 {
                    Some(*k1)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match (&self.inf, &self.sup) {
            (Bound::Open(k1), Bound::Open(k2)) => k1 == k2,
            _ => false,
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "∅")
        } else if self.is_singleton() {
            write!(f, "{{{:6.2}}}", self.get_singleton().unwrap())
        } else {
            let infstr = match self.inf {
                Bound::Open(k) => format!("({k:6.2}"),
                Bound::Closed(k) => format!("[{k:6.2}"),
                Bound::NegativeInfinity => "(-∞".to_owned(),
                Bound::PositiveInfinity => "(+∞".to_owned(),
            };

            let supstr = match self.sup {
                Bound::Open(k) => format!("{k:6.2})"),
                Bound::Closed(k) => format!("{k:6.2}]"),
                Bound::NegativeInfinity => "-∞)".to_owned(),
                Bound::PositiveInfinity => "+∞)".to_owned().to_owned(),
            };

            write!(f, "{},{}", infstr, supstr)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fmt0() {
        let a = Interval::empty();
        assert_eq!(format!("{a}"), "∅");
    }
    #[test]
    fn test_fmt1() {
        let a = Interval::singleton(5.4);
        assert_eq!(format!("{a}"), "{  5.40}");
    }
    #[test]
    fn test_fmt2() {
        let a = Interval::new(Bound::NegativeInfinity, Bound::PositiveInfinity);
        assert_eq!(format!("{a}"), "(-∞,+∞)");
    }
    #[test]
    fn test_fmt4() {
        let a = Interval::new(Bound::NegativeInfinity, Bound::Open(-5.));
        assert_eq!(format!("{a}"), "(-∞, -5.00)");
    }
    #[test]
    fn test_fmt5() {
        let a = Interval::new(Bound::NegativeInfinity, Bound::Closed(-5.));
        assert_eq!(format!("{a}"), "(-∞, -5.00]");
    }
    #[test]
    fn test_fmt6() {
        let a = Interval::new(Bound::Closed(42.), Bound::PositiveInfinity);
        assert_eq!(format!("{a}"), "[ 42.00,+∞)");
    }
    #[test]
    fn test_fmt7() {
        let a = Interval::new(Bound::Open(42.), Bound::PositiveInfinity);
        assert_eq!(format!("{a}"), "( 42.00,+∞)");
    }
    #[test]
    #[should_panic]
    fn test_fmt9() {
        let a = Interval::new(Bound::Open(42.), Bound::Open(-5.));
        assert_eq!(format!("{a}"), "( -5.00, 42.00)");
    }
    #[test]
    fn test_fmt10() {
        let a = Interval::new(Bound::Open(-5.), Bound::Open(42.));
        assert_eq!(format!("{a}"), "( -5.00, 42.00)");
    }
    #[test]
    #[should_panic]
    fn test_fmt11() {
        let a = Interval::new(Bound::Closed(42.), Bound::Closed(-5.));
        assert_eq!(format!("{a}"), "[ -5.00, 42.00]");
    }
    #[test]
    #[should_panic]
    fn test_fmt12() {
        let a = Interval::new(Bound::Closed(42.), Bound::Open(-5.));
        assert_eq!(format!("{a}"), "( -5.00, 42.00]");
    }
    #[test]
    #[should_panic]
    fn test_fmt13() {
        let a = Interval::new(Bound::Open(42.), Bound::Closed(-5.));
        assert_eq!(format!("{a}"), "[ -5.00, 42.00)");
    }
}
