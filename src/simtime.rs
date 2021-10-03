use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub struct SimTimeDelta {
    delta: Duration
}

impl From<Duration> for SimTimeDelta {
    fn from(delta: Duration)  -> Self {
        SimTimeDelta {delta}
    }
}

impl SimTimeDelta {
    #[deprecated]
    pub const fn from_duration(delta: Duration) -> Self {
        SimTimeDelta {delta}
    }

    pub const fn from(delta: Duration) -> Self {
        SimTimeDelta {delta}
    }
}

pub const NO_DELTA: SimTimeDelta = SimTimeDelta { delta: Duration::from_secs(0) };

#[derive(Default, Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub struct SimTime {
    time: Duration,
}

impl std::ops::Add<SimTimeDelta> for SimTime {
    type Output = SimTime;

    fn add(self, _rhs: SimTimeDelta) -> SimTime {
        SimTime { time: self.time + _rhs.delta }
    }
}

impl std::ops::Sub<SimTime> for SimTime {
    type Output = SimTimeDelta;

    fn sub(self, _rhs: SimTime) -> SimTimeDelta {
        SimTimeDelta::from(self.time - _rhs.time)
    }
}

impl SimTime {
    pub(super) fn advance_to(&mut self, new_time: SimTime) {
        assert!(self.time <= new_time.time, "time mismatch: {:?} {:?}", self, new_time);
        self.time = new_time.time;
    }

    pub fn is_zero(&self) -> bool {
        return self.time.as_secs() == 0 && self.time.as_nanos() == 0;
    }

    pub fn as_rounds(&self) -> u64 {
        return self.time.as_secs();
    }

    // TODO: implement meaningful display for sim_time;

    pub fn as_millis(&self) -> u128 {
        return self.time.as_millis()
    }
}
