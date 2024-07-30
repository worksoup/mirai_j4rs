use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{utils::instance_is_null, TryFromInstanceTrait};
use mj_helper_macro::mj_event;
use std::fmt::{Display, Formatter};

use crate::contact::Group;
use crate::event::{BotEventTrait, FriendInfoChangeEventTrait};
use crate::message::MessageHashCodeTrait;
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct NewFriendRequestEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> NewFriendRequestEvent<B> {
    pub fn accept(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm
            .invoke(&self.instance, "accept", InvocationArg::empty())
            .unwrap();
    }
    pub fn reject(&self, black_list: bool) {
        let jvm = Jvm::attach_thread().unwrap();
        let black_list = InvocationArg::try_from(black_list)
            .unwrap()
            .into_primitive()
            .unwrap();
        let _ = jvm.invoke(&self.instance, "reject", &[black_list]).unwrap();
    }
    pub fn get_event_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getEventId", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_from_group(&self) -> Option<Group<B>> {
        let jvm = Jvm::attach_thread().unwrap();
        let group = jvm
            .invoke(&self.instance, "getFromGroup", InvocationArg::empty())
            .unwrap();
        if !instance_is_null(&group) {
            Group::try_from_instance(group).ok()
        } else {
            None
        }
    }
    pub fn get_from_group_id(&self) -> Option<i64> {
        let jvm = Jvm::attach_thread().unwrap();
        let id: i64 = jvm
            .to_rust(
                jvm.invoke(&self.instance, "getFromGroupId", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap();
        if id != 0 {
            Some(id)
        } else {
            None
        }
    }
    pub fn get_from_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getFromId", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_from_nick(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getFromNick", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_from_message(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getMessage", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
}
impl<B: BotBackend> Display for NewFriendRequestEvent<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            {
                let jvm = Jvm::attach_thread().unwrap();
                jvm.to_rust::<String>(
                    jvm.invoke(&self.instance, "toString", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap()
            }
            .as_str(),
        )
    }
}
impl<B: BotBackend> MessageHashCodeTrait for NewFriendRequestEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for NewFriendRequestEvent<B> {}

impl<B: BotBackend> FriendInfoChangeEventTrait<B> for NewFriendRequestEvent<B> {}
