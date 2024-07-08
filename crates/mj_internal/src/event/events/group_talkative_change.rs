use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait, GroupEventTrait};

#[mj_event]
pub struct GroupTalkativeChangeEvent {
    instance: Instance,
}
impl BotPassiveEventTrait for GroupTalkativeChangeEvent {}
impl GroupEventTrait for GroupTalkativeChangeEvent {}

impl BotEventTrait for GroupTalkativeChangeEvent {}
