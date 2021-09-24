use crate::keys::{ComponentId, ChannelId};
use crate::scheduler::{RoundScheduler, ROUND_DELTA};
use std::any::Any;

// as of now there is only one possible implementation of channel
// as bidirectional channel

// for the time when there will be another channel implementation

pub trait ChannelTrait {
    fn accept_message_from(&mut self,
                        source: ComponentId,
                        message: Box<dyn Any>,
                        scheduler: &mut RoundScheduler);
}

// TODO move into synch subfolder

pub mod synch {
    use crate::keys::{ChannelId, ComponentId};
    use crate::scheduler::{RoundScheduler, ROUND_DELTA};
    use std::any::Any;
    use crate::channel::ChannelTrait;

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
                            scheduler: &mut RoundScheduler) {

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

    impl Channel {
        pub fn new(id: ChannelId, p0: ComponentId, p1: ComponentId) -> Self {
            Channel{ id, left: p0, right: p1 }
        }
    }
}