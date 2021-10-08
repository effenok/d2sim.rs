use d2simrs::basicnet::types::InterfaceId;

pub use crate::simpledv::addr::HostAddr;

// normally we need to configure each router interface to use EIGRP
// however we will skip this for simplicity and run EIGRP on each interface
// not configured otherwise

#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    pub if_id: InterfaceId,
    pub advertise_addr: HostAddr,
}

#[derive(Debug)]
pub struct Config {
    interfaces: Vec<InterfaceConfig>,
}

// NOTE: according to docs Vec::new() does not actually allocate memory

impl Default for Config {
    fn default() -> Self {
        Config {
            interfaces: Vec::new()
        }
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        // eprintln!("called clone");
        Config {interfaces: self.interfaces.clone()}
    }
}

impl Config {
    pub fn new() -> Self {
        Config {
            interfaces: Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.interfaces.len() == 0
    }

    pub fn add_interface(&mut self, if_id: InterfaceId, addr: HostAddr) {
        self.interfaces.push(InterfaceConfig {
            if_id,
            advertise_addr: addr,
        })
    }

    pub fn has_interface(&self, if_id: InterfaceId) -> bool {
        assert!(self.interfaces.len() <= 1, "config with more than one interface is not implemented");

        self.interfaces.len() != 0 && self.interfaces[0].if_id == if_id
    }

    pub fn get_config(&self, if_id: InterfaceId) -> &InterfaceConfig {
        assert!(self.interfaces.len() <= 1, "config with more than one interface is not implemented");

        assert!(self.interfaces.len() == 1);
        assert!(self.interfaces[0].if_id == if_id);

        &self.interfaces[0]
    }
}

