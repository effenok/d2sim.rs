use crate::keys::{ComponentId, ChannelId};
use crate::scheduler::{Scheduler};
use std::any::Any;

// as of now there is only one possible implementation of channel
// as bidirectional channel

// for the time when there will be another channel implementation

pub trait ChannelTrait {
    fn accept_message_from(&mut self,
                        source: ComponentId,
                        message: Box<dyn Any>,
                        scheduler: &mut Scheduler);
}

pub trait ChannelBuilder {
    type C : ChannelTrait;

    fn build_channel(&self, c: ChannelId, p0: ComponentId, p1: ComponentId) -> Self::C;
}

// TODO move into synch subfolder
pub mod synch {
    use crate::keys::{ChannelId, ComponentId};
    use crate::scheduler::{Scheduler, ROUND_DELTA};
    use std::any::Any;
    use crate::channel::{ChannelTrait, ChannelBuilder};

    #[derive(Debug)]
    pub struct Channel {
        pub id: ChannelId,
        pub left: ComponentId,
        pub right: ComponentId,
    }

    impl ChannelTrait for Channel {

        fn accept_message_from(&mut self,
                            source: ComponentId,
                            message: Box<dyn Any>,
                            scheduler: &mut Scheduler) {

            let dst : ComponentId;

            if source == self.left {
                dst = self.right;
            } else if source == self.right {
                dst = self.left;
            } else {
                panic! ("unknown source {:?} for channel {:?}", source, self);
            }

            // TODO: this channel only works for synchronous networks
            scheduler.sched_receive_msg(ROUND_DELTA, dst, self.id, message);
        }
    }

    // impl Channel {
    //     fn new(id: ChannelId, p0: ComponentId, p1: ComponentId) -> Self {
    //         Channel{ id, left: p0, right: p1 }
    //     }
    // }

    pub struct SynchChannelBuilder {}

    impl ChannelBuilder for SynchChannelBuilder{
        type C = Channel;

        fn build_channel(&self, id: ChannelId, p0: ComponentId, p1: ComponentId) -> Self::C {
            Channel{ id, left: p0, right: p1 }
        }
    }
}

pub mod asynch {
    use crate::keys::{ChannelId, ComponentId};
    use crate::scheduler::{Scheduler, SimTimeDelta, NO_DELTA};
    use std::any::Any;
    use crate::channel::{ChannelTrait, ChannelBuilder};


    #[derive(Debug)]
    pub struct Channel {
        pub id: ChannelId,
        pub left: ComponentId,
        pub right: ComponentId,
        pub delay: SimTimeDelta,
    }

    impl ChannelTrait for Channel {

        fn accept_message_from(&mut self,
                               source: ComponentId,
                               message: Box<dyn Any>,
                               scheduler: &mut Scheduler) {

            let dst : ComponentId;

            if source == self.left {
                dst = self.right;
            } else if source == self.right {
                dst = self.left;
            } else {
                panic! ("unknown source {:?} for channel {:?}", source, self);
            }

            scheduler.sched_receive_msg(self.delay, dst, self.id, message);
        }
    }

    pub struct AsynchChannelBuilder {
        delay: SimTimeDelta
    }
    
    impl Default for AsynchChannelBuilder {
        fn default() -> Self {
            AsynchChannelBuilder { delay: NO_DELTA } 
        }
    }

    impl ChannelBuilder for AsynchChannelBuilder{
        type C = Channel;

        fn build_channel(&self, id: ChannelId, p0: ComponentId, p1: ComponentId) -> Self::C {
            Channel{ id, left: p0, right: p1, delay: self.delay}
        }
    }

    impl AsynchChannelBuilder {

        pub fn with_delay(&mut self, delay: std::time::Duration) -> &mut Self {
            self.delay = SimTimeDelta::from_duration(delay);
            self
        }

    }

}