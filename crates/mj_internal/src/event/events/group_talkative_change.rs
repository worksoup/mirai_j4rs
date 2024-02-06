use crate::event::{BotEventTrait, BotPassiveEventTrait, GroupEventTrait, MiraiEventTrait};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupTalkativeChangeEvent")]
pub struct GroupTalkativeChangeEvent {
    instance: Instance,
}
impl BotPassiveEventTrait for GroupTalkativeChangeEvent {}
impl GroupEventTrait for GroupTalkativeChangeEvent {}

impl BotEventTrait for GroupTalkativeChangeEvent {}
impl MiraiEventTrait for GroupTalkativeChangeEvent {}
