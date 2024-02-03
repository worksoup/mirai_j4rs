use crate::{
    contact::{AnonymousMember, Friend, Group, Member, NormalMember},
    event::event_trait::{MessageEventTrait, MiraiEventTrait},
};
use j4rs::{Instance, Jvm};
use mj_base::env::FromInstance;
use mj_macro::{java_type, GetInstanceDerive};

#[derive(GetInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.GroupMessageEvent")]
pub struct GroupMessageEvent {
    instance: Instance,
}

impl GroupMessageEvent {}

impl FromInstance for GroupMessageEvent {
    fn from_instance(instance: Instance) -> Self {
        GroupMessageEvent { instance }
    }
}

// 这个特征实现了一个 event 所需要的函数。
impl MiraiEventTrait for GroupMessageEvent {}

// 实现了 message 所需要的函数。
impl MessageEventTrait for GroupMessageEvent {
    type UserItem = Member;

    fn get_sender(&self) -> Self::UserItem {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getSender", &[]).unwrap();
        let special_title: String = jvm
            .to_rust(
                jvm.invoke(
                    // 下面之所以转换是因为 java 中这个函数似乎返回了 `net.mamoe.mirai.contact.User`, 是没有 `getSpecialTitle` 这个方法的。
                    &jvm.cast(&instance, "net.mamoe.mirai.contact.Member")
                        .unwrap(),
                    "getSpecialTitle",
                    &[],
                )
                .unwrap(),
            )
            .unwrap();
        match special_title.as_str() {
            "匿名" => {
                println!("匿名成员");
                Member::AnonymousMember(AnonymousMember::from_instance(instance))
            }
            _ => {
                println!("普通成员");
                Member::NormalMember(NormalMember::from_instance(instance))
            }
        }
    }

    type ContactItem = Group;

    fn get_subject(&self) -> Self::ContactItem {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getSubject", &[]).unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        Group { bot, instance, id }
    }
}

#[derive(GetInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.FriendMessageEvent")]
pub struct FriendMessageEvent {
    instance: Instance,
}

impl FriendMessageEvent {}

impl FromInstance for FriendMessageEvent {
    fn from_instance(instance: Instance) -> Self {
        FriendMessageEvent { instance }
    }
}

impl MiraiEventTrait for FriendMessageEvent {}

impl MessageEventTrait for FriendMessageEvent {
    type UserItem = Friend;
    fn get_sender(&self) -> Self::UserItem {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getSender", &[]).unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        Friend { bot, instance, id }
    }
    type ContactItem = Friend;

    fn get_subject(&self) -> Friend {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getSubject", &[]).unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        Friend { bot, instance, id }
    }
}

// TODO
pub struct GroupTempMessageEvent {}

// TODO
pub struct StrangerMessageEvent {}

// TODO
pub struct OtherClientMessageEvent {}

// TODO
pub trait MessagePreSendEventTrait {}

// TODO
pub struct GroupMessagePreSendEvent {}

// TODO
pub struct FriendMessagePreSendEvent {}

// TODO
pub struct GroupTempMessagePreSendEvent {}

// TODO
pub struct StrangerMessagePreSendEvent {}

// TODO
pub struct OtherClientMessagePreSendEvent {}

// TODO
pub trait MessagePostSendEventTrait {}

// TODO
pub struct GroupMessagePostSendEvent {}

// TODO
pub struct FriendMessagePostSendEvent {}

// TODO
pub struct GroupTempMessagePostSendEvent {}

// TODO
pub struct StrangerMessagePostSendEvent {}

// TODO
pub struct OtherClientMessagePostSendEvent {}

// TODO
pub trait MessageRecallTrait {}

// TODO
pub enum MessageRecall {
    FriendRecall,
    GroupRecall,
    TempRecall,
}

// TODO
pub struct BeforeImageUploadEvent {}

// TODO
pub enum ImageUploadEvent {
    Succeed,
    Failed,
}

// TODO
pub struct NudgeEvent {}

// TODO
pub trait MessageSyncEvent {}