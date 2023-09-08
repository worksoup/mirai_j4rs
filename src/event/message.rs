use crate::event::event_trait::MiraiEventTrait;
use crate::{
    contact::{bot::Bot, group::Group, AnonymousMember, Friend, Member, NormalMember},
    message::MessageChain,
};
use contact_derive::{GetClassTypeDerive, GetInstanceDerive};
use j4rs::{Instance, Jvm};

pub trait MessageEventTrait
    where
        Self: MiraiEventTrait,
{
    fn get_bot(&self) -> Bot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&self.get_instance(), "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&bot, "getId", &[]).unwrap())
            .unwrap();
        Bot { bot, id }
    }
    fn get_message(&self) -> MessageChain {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "getMessage", &[]).unwrap();
        MessageChain { instance }
    }
    type UserItem;
    fn get_sender(&self) -> Self::UserItem;
    fn get_sender_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.get_instance(), "getSenderName", &[])
                .unwrap(),
        )
            .unwrap()
    }
    fn get_source(&self) -> () {
        todo!("net.mamoe.mirai.message.data.OnlineMessageSource.Incoming")
    }
    type ContactItem;
    fn get_subject(&self) -> Self::ContactItem;
    fn get_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.get_instance(), "getTime", &[]).unwrap())
            .unwrap()
    }
}

#[derive(GetInstanceDerive, GetClassTypeDerive)]
pub struct GroupMessageEvent {
    instance: Instance,
}

impl GroupMessageEvent {
    // 该函数被 GetClassTypeDerive 宏使用。该宏实现了 GetClassTypeTrait。
    // 这个特征可以返回 java 中 Class 对象，监听事件的时候用。
    // 为了做泛型搞的。之后可能会改动。
    /// 获取 java 中的类名。TODO: 需要移除该函数。该函数的引入是由于 j4rs 旧版本中的 bug.
    /// `getClass` 方法属于每一个 Object, 但由于 bug, 无法通过 j4rs 直接调用之。
    /// 见 https://github.com/astonbitecode/j4rs/issues/71
    fn get_class_name() -> String {
        "net.mamoe.mirai.event.events.GroupMessageEvent".to_owned()
    }
}

// 这个特征实现了一个 event 所需要的函数。
impl MiraiEventTrait for GroupMessageEvent {
    fn from_instance(instance: Instance) -> Self {
        GroupMessageEvent { instance }
    }
}

// 实现了 message 所需要的函数。
impl MessageEventTrait for GroupMessageEvent {
    type UserItem = Member;

    fn get_sender(&self) -> Self::UserItem {
        // j4rs 旧版本中有 bug, 所以只能如下注释中的写法。见 https://github.com/astonbitecode/j4rs/issues/71
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getSender", &[]).unwrap();
        let bot = jvm
            .invoke(
                // &jvm.cast(&instance, "net.mamoe.mirai.contact.Contact")             // j4rs <= 0.17.1
                // .unwrap(),                                                          // j4rs <= 0.17.1
                &instance,
                "getBot",
                &[],
            )
            .unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        let special_title: String = jvm
            .to_rust(
                jvm.invoke(
                    // 下面两行之所以转换是因为 java 中这个函数似乎返回了 `net.mamoe.mirai.contact.User`, 是没有 `getSpecialTitle` 这个函数的。
                    &jvm.cast(&instance, "net.mamoe.mirai.contact.Member")
                        .unwrap(),
                    // &instance,
                    "getSpecialTitle",
                    &[],
                )
                    .unwrap(),
            )
            .unwrap();
        match special_title.as_str() {
            "匿名" => {
                println!("匿名成员");
                Member::AnonymousMember(AnonymousMember { bot, instance, id })
            }
            _ => {
                println!("普通成员");
                Member::NormalMember(NormalMember { bot, instance, id })
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

#[derive(GetInstanceDerive, GetClassTypeDerive)]
pub struct FriendMessageEvent {
    instance: Instance,
}

impl FriendMessageEvent {
    fn get_class_name() -> String {
        "net.mamoe.mirai.event.events.FriendMessageEvent".to_owned()
    }
}

impl MiraiEventTrait for FriendMessageEvent {
    fn from_instance(instance: Instance) -> Self {
        FriendMessageEvent { instance }
    }
}

impl MessageEventTrait for FriendMessageEvent {
    type UserItem = Friend;
    fn get_sender(&self) -> Self::UserItem {
        todo!()
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

pub struct GroupTempMessageEvent {}

pub struct StrangerMessageEvent {}

pub struct OtherClientMessageEvent {}

pub trait MessagePreSendEventTrait {}

pub struct GroupMessagePreSendEvent {}

pub struct FriendMessagePreSendEvent {}

pub struct GroupTempMessagePreSendEvent {}

pub struct StrangerMessagePreSendEvent {}

pub struct OtherClientMessagePreSendEvent {}

pub trait MessagePostSendEventTrait {}

pub struct GroupMessagePostSendEvent {}

pub struct FriendMessagePostSendEvent {}

pub struct GroupTempMessagePostSendEvent {}

pub struct StrangerMessagePostSendEvent {}

pub struct OtherClientMessagePostSendEvent {}

pub trait MessageRecallTrait {}

pub enum MessageRecall {
    FriendRecall,
    GroupRecall,
    TempRecall,
}

pub struct BeforeImageUploadEvent {}

pub enum ImageUploadEvent {
    Succeed,
    Failed,
}

pub struct NudgeEvent {}

pub trait MessageSyncEvent {}
