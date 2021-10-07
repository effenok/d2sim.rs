use d2simrs::simtime::SimTimeDelta;
use std::time::Duration;

pub(super) const HELLO_INTERVAL_SEC: u64 = 5;
pub(super) const HELLO_INTERVAL: SimTimeDelta = SimTimeDelta::from(Duration::from_secs(HELLO_INTERVAL_SEC));
pub(super) const HOLD_TIME: SimTimeDelta = SimTimeDelta::from(Duration::from_secs(3 * HELLO_INTERVAL_SEC));
// default - 3 times hello interval
pub(super) const MAX_HOP_COUNT: usize = 15;