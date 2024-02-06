use crate::event::{BotEventTrait, BotPassiveEventTrait, GroupEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct GroupTalkativeChangeEvent {
    instance: Instance,
}
impl BotPassiveEventTrait for GroupTalkativeChangeEvent {}
impl GroupEventTrait for GroupTalkativeChangeEvent {}

impl BotEventTrait for GroupTalkativeChangeEvent {}
