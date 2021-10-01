use crate::scheduler::Scheduler;
use crate::simtime::SimTime;
use crate::environment::Environment;

// TODO: this is not THREAD SAFE
// if it is possible to issue a warning on multithreaded access

static mut INIT_GUARD: bool = false;
pub static mut SIM: SimVars = SimVars { scheduler: std::ptr::null_mut() };

pub struct SimVars {
    scheduler: *mut Scheduler
}

impl SimVars {
    pub fn init(&mut self) {
        let s = Box::new(Scheduler::new());
        self.scheduler = Box::leak(s);
        unsafe {INIT_GUARD = true; }
    }
}

pub fn sim_sched() -> &'static mut Scheduler {
     unsafe {
         assert!(INIT_GUARD);
         &mut *SIM.scheduler
    }
}

pub fn sim_time() -> SimTime {
    unsafe {
        assert!(INIT_GUARD);
        (*SIM.scheduler).get_curr_time().clone()
    }
}

pub fn sim_env() -> &'static mut Environment {
    unsafe {
        assert!(INIT_GUARD);
        &mut (*SIM.scheduler).env
    }
}

