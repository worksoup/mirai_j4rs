use crate::contact::{AnonymousMember, Group, Member, NormalMember};
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, FriendMessageSyncEvent, MessageEventTrait,
    MiraiEventTrait, OtherClientEventTrait,
};
use j4rs::{Instance, Jvm};
use mj_base::env::FromInstance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupMessageEvent")]
pub struct GroupMessageEvent {
    instance: Instance,
}

impl GroupMessageEvent {}

// 这个特征实现了一个 event 所需要的函数。
impl MiraiEventTrait for GroupMessageEvent {}

// 实现了 message 所需要的函数。
impl MessageEventTrait<Member, Group> for GroupMessageEvent {
    fn get_sender(&self) -> Member {
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
        // TODO: 这样的转换存在非常情况，并不科学。
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

    fn get_subject(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getSubject", &[]).unwrap();
        Group::from_instance(instance)
    }
}
impl OtherClientEventTrait for GroupMessageEvent {}

impl BotEventTrait for GroupMessageEvent {}

impl BotPassiveEventTrait for GroupMessageEvent {}
