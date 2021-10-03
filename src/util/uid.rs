use rand::Rng;

use std::collections::HashSet;
use std::fmt;

//TODO: generic UID?

#[derive(Debug, PartialEq, Eq, PartialOrd, Default, Copy, Clone)]
pub struct UniqueId(pub usize);

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for UniqueId {
    fn from(uid: usize) -> Self {
        UniqueId { 0: uid }
    }
}

#[derive(Default, Debug)]
pub struct UIdGenSequential {
    next_uid: usize,
}

impl UIdGenSequential {
    #[allow(dead_code)]
    pub fn new(initial_uid: usize) -> Self {
        UIdGenSequential {next_uid: initial_uid,}
    }
    #[allow(dead_code)]
    pub fn generate_uid(&mut self) -> UniqueId {
        let uid = UniqueId(self.next_uid);
        self.next_uid+=1;
        uid
    }
}

#[derive( Debug)]
pub struct UIdGenRandom {
    max: usize,
    generated_ids: HashSet<usize>,
}

impl Default for UIdGenRandom {
	fn default() -> Self {
        UIdGenRandom { max: std::usize::MAX, generated_ids: HashSet::new() }
	}
}

impl UIdGenRandom {
    #[allow(dead_code)]
    pub fn new(max: usize) -> Self {
        UIdGenRandom {max, generated_ids: HashSet::new()}
    }
    #[allow(dead_code)]
    pub fn generate_uid(&mut self) -> UniqueId {
        let mut uid_val;
        loop {
            uid_val = rand::thread_rng().gen_range(0..self.max);
            if !self.generated_ids.contains(&uid_val) {break;}
        }

        self.generated_ids.insert(uid_val);
        UniqueId(uid_val)
    }
}

// TODO: rng that validates that generated UIDs are unique
// TODO: check if i can do something like inheritance

//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::uid::UniqueId;

    #[test]
    fn test_sequential1() {
        let mut gen = super::UIdGenSequential::default();
        println!("{:?}", gen);
        for i in 0..5 {
            let uid = gen.generate_uid();
            assert_eq!(uid, UniqueId(i));
            println!("{:?}", uid);
        }
        println!("{:?}", gen);
    }

    #[test]
    fn test_sequential2() {
        const INITIAL_UID: usize = 100;
        let mut gen = super::UIdGenSequential::new(INITIAL_UID);
        println!("{:?}", gen);
        for i in 0..5 {
            let uid = gen.generate_uid();
            assert_eq!(uid, UniqueId(i+ INITIAL_UID));
            println!("{:?}", uid);
        }
        println!("{:?}", gen);
    }

    #[test]
    fn test_random() {
        // TODO: figure out how to provide a seed to RNG and then
        // write a test for random values
        const MAX_UID: usize = 100;
        let mut gen = super::UIdGenRandom::new(MAX_UID);
        println!("{:?}", gen);
        for _ in 0..5 {
            let uid = gen.generate_uid();
            // assert_eq!(uid, UniqueId(i));
            println!("{:?}", uid);
        }
        println!("{:?}", gen);
    }
}