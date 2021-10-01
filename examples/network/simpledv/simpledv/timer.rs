use crate::router::InternalEvent;
use crate::types::InterfaceId;

#[derive(Debug)]
pub(super) struct HelloTimer {
    pub(super) interface_id: InterfaceId,
}

impl InternalEvent {
    pub(super) fn new_hello_timer(interface_id: InterfaceId) -> Box<Self> {
        Box::new(InternalEvent::L3TimerEvent(Box::new(HelloTimer {
            interface_id
        })))
    }

    pub(super) fn from_hello_timer(timer: Box<HelloTimer>) -> Box<Self> {
        Box::new(InternalEvent::L3TimerEvent(timer))
    }
}

#[derive(Debug)]
pub(super) struct NeighborHoldTimer {
    pub(super) interface_id: InterfaceId,
}

impl InternalEvent {
    pub(super) fn new_hold_timer(interface_id: InterfaceId) -> Box<Self> {
        Box::new(InternalEvent::L3TimerEvent(Box::new(NeighborHoldTimer {
            interface_id
        })))
    }

    pub(super) fn from_hold_timer(timer: Box<NeighborHoldTimer>) -> Box<Self> {
        Box::new(InternalEvent::L3TimerEvent(timer))
    }
}