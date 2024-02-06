use crate::contact::Group;
use crate::event::{BotEventTrait, FriendInfoChangeEventTrait};
use crate::message::MessageHashCodeTrait;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::{env::FromInstance, utils::instance_is_null};
use mj_macro::mj_event;

#[mj_event]
pub struct NewFriendRequestEvent {
    instance: Instance,
}

impl NewFriendRequestEvent {
    pub fn accept(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "accept", &[]).unwrap();
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
        jvm.to_rust(jvm.invoke(&self.instance, "getEventId", &[]).unwrap())
            .unwrap()
    }
    pub fn get_from_group(&self) -> Option<Group> {
        let jvm = Jvm::attach_thread().unwrap();
        let group = jvm.invoke(&self.instance, "getFromGroup", &[]).unwrap();
        if !instance_is_null(&group) {
            Some(Group::from_instance(group))
        } else {
            None
        }
    }
    pub fn get_from_group_id(&self) -> Option<i64> {
        let jvm = Jvm::attach_thread().unwrap();
        let id: i64 = jvm
            .to_rust(jvm.invoke(&self.instance, "getFromGroupId", &[]).unwrap())
            .unwrap();
        if id != 0 {
            Some(id)
        } else {
            None
        }
    }
    pub fn get_from_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getFromId", &[]).unwrap())
            .unwrap()
    }
    pub fn get_from_nick(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getFromNick", &[]).unwrap())
            .unwrap()
    }
    pub fn get_from_message(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getMessage", &[]).unwrap())
            .unwrap()
    }
    pub fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "toString", &[]).unwrap())
            .unwrap()
    }
}

impl MessageHashCodeTrait for NewFriendRequestEvent {}

impl BotEventTrait for NewFriendRequestEvent {}

impl FriendInfoChangeEventTrait for NewFriendRequestEvent {}
