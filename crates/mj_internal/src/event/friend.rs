use crate::{
    contact::Group,
    event::event_trait::{BotEventTrait, FriendInfoChangedEvent, MiraiEventTrait},
    message::MessageHashCodeTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::{
    env::{FromInstance, GetClassTypeTrait},
    utils::instance_is_null,
};
use mj_macro::{java_type, AsInstanceDerive, GetInstanceDerive};

pub struct FriendRemarkChangeEvent {}

pub struct FriendAddEvent {}

pub struct FriendDeleteEvent {}

#[derive(GetInstanceDerive, AsInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.NewFriendRequestEvent")]
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

impl FromInstance for NewFriendRequestEvent {
    fn from_instance(instance: Instance) -> Self {
        NewFriendRequestEvent { instance }
    }
}

impl MiraiEventTrait for NewFriendRequestEvent {}

impl BotEventTrait for NewFriendRequestEvent {}

impl FriendInfoChangedEvent for NewFriendRequestEvent {}

pub struct FriendAvatarChangedEvent {}

pub struct FriendNickChangedEvent {}

pub struct FriendInputStatusChangedEvent {}
