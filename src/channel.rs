use crate::keys::{ComponentId, ChannelId};
use std::any::Any;

pub trait ChannelTrait {
    fn accept_message_from(&mut self,
                        source: ComponentId,
                        message: Box<dyn Any>,
    );
}

pub trait ChannelBuilder {
    type C : ChannelTrait;

    fn build_channel(&self, c: ChannelId, p0: ComponentId, p1: ComponentId) -> Self::C;
}