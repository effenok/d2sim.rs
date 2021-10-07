use std::fmt;

use crate::simpledv::constants::MAX_HOP_COUNT;

const INFINITY_HOP_COUNT: usize = MAX_HOP_COUNT;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metric {
    hop_count: usize,
}

impl Default for Metric {

    /// create metric with value infinity
    fn default() -> Self {
        Metric { hop_count: INFINITY_HOP_COUNT }
    }
}

impl std::ops::Add<Metric> for Metric {
    type Output = Metric;

    fn add(self, _rhs: Metric) -> Metric {
        if self.hop_count == INFINITY_HOP_COUNT || _rhs.hop_count == INFINITY_HOP_COUNT {
            Metric::default()
        } else if self.hop_count + _rhs.hop_count >= INFINITY_HOP_COUNT {
            Metric::default()
        } else {
            Metric { hop_count: self.hop_count + _rhs.hop_count }
        }
    }
}

impl Metric {
    pub const ONE_HOP: Metric = Metric { hop_count: 1 };
    pub const INFINITY: Metric = Metric { hop_count: INFINITY_HOP_COUNT};

    pub fn new_zero() -> Self {
        Metric { hop_count: 0 }
    }

    pub(crate) fn is_infinity(&self) -> bool {
        self.hop_count >= INFINITY_HOP_COUNT
    }
}

impl fmt::Debug for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const INF_CHAR: char = '\u{221E}';

        if self.is_infinity() {
            f.debug_struct("Metric")
                .field("hop_count", &INF_CHAR)
                .finish()
        } else {
            f.debug_struct("Metric")
                .field("hop_count", &self.hop_count)
                .finish()
        }

    }
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_infinity() {
            write!(f, "inf")
        } else {
            write!(f, "{}", self.hop_count)
        }
    }
}
