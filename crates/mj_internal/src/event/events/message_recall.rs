use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, GroupEventTrait, GroupOperableEventTrait};

pub trait MessageRecallTrait: BotEventTrait {}
#[mj_event("event.events.MessageRecallEvent$FriendRecall")]
pub struct FriendRecall {
    instance: Instance,
}

impl BotEventTrait for FriendRecall {}

impl MessageRecallTrait for FriendRecall {}

#[mj_event("event.events.MessageRecallEvent$GroupRecall")]
pub struct GroupRecall {
    instance: Instance,
}

impl BotEventTrait for GroupRecall {}

impl MessageRecallTrait for GroupRecall {}

impl GroupEventTrait for GroupRecall {}

impl GroupOperableEventTrait for GroupRecall {}

#[mj_event]
pub struct MessageRecallEvent {
    instance: Instance,
}

impl BotEventTrait for MessageRecallEvent {}

impl MessageRecallTrait for MessageRecallEvent {}
