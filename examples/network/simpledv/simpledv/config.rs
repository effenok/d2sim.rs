pub use crate::simpledv::addr::HostAddr;
use crate::types::InterfaceId;

// normally we need to configure each router interface to use EIGRP
// however we will skip this for simplicity and run EIGRP on each interface
// not configured otherwise

pub struct InterfaceConfig {
    pub if_id: InterfaceId,
    pub advertise_addr: HostAddr,
}

// TODO: this should be a vector
pub struct Config {
    interfaces: Vec<InterfaceConfig>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            interfaces: Vec::new()
        }
    }

    pub fn with_interface(if_id: InterfaceId, addr: HostAddr) {
        let mut cfg = Config {
            interfaces: Vec::with_capacity(1)
        };

        cfg.interfaces.push(InterfaceConfig {
            if_id,
            advertise_addr: addr,
        })
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

