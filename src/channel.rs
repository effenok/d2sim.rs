use crate::keys::{ComponentId, ChannelId};
use crate::scheduler::{Scheduler};
use std::any::Any;

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