use crate::contact::Friend;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, FriendEventTrait, MessageEventTrait, MiraiEventTrait,
    OtherClientEventTrait, UserEventTrait, UserMessageEventTrait,
};
use j4rs::{Instance, Jvm};
use mj_base::env::FromInstance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.FriendMessageEvent")]
pub struct FriendMessageEvent {
    instance: Instance,
}

impl FriendMessageEvent {}

impl MiraiEventTrait for FriendMessageEvent {}

impl MessageEventTrait<Friend, Friend> for FriendMessageEvent {}
impl OtherClientEventTrait for FriendMessageEvent {}

impl UserMessageEventTrait<Friend, Friend> for FriendMessageEvent {}
impl UserEventTrait for FriendMessageEvent {}

impl FriendEventTrait for FriendMessageEvent {}

impl BotEventTrait for FriendMessageEvent {}

impl BotPassiveEventTrait for FriendMessageEvent {}
