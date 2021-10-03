use std::fmt;

use crate::simpledv::constants::MAX_METRIC;

const INFINITY: usize = MAX_METRIC;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metric {
    hop_count: usize,
}

impl Default for Metric {
    fn default() -> Self {
        Metric { hop_count: INFINITY }
    }
}

impl std::ops::Add<Metric> for Metric {
    type Output = Metric;

    fn add(self, _rhs: Metric) -> Metric {
        if self.hop_count == INFINITY || _rhs.hop_count == INFINITY {
            Metric::default()
        } else if self.hop_count + _rhs.hop_count >= INFINITY {
            Metric::default()
        } else {
            Metric { hop_count: self.hop_count + _rhs.hop_count }
        }
    }
}

impl Metric {
    pub const ONE_HOP: Metric = Metric { hop_count: 1 };

    pub fn new_zero() -> Self {
        Metric { hop_count: 0 }
    }
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hop_count >= INFINITY {
            write!(f, "inf")
        } else {
            write!(f, "{}", self.hop_count)
        }
    }
}
