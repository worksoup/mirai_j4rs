use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::TryFromInstanceTrait;
use mj_helper_macro::mj_event;
use std::fmt::{Display, Formatter};

use crate::contact::{Bot, Group, NormalMember};
use crate::event::{BotEventTrait, MiraiEventTrait};
use crate::message::MessageHashCodeTrait;
use crate::utils::backend::BotBackend;

// TODO: BaseGroupMemberInfoChangeEvent
#[mj_event]
pub struct MemberJoinRequestEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MemberJoinRequestEvent<B> {
    pub fn accept(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm
            .invoke(&self.instance, "getBot", InvocationArg::empty())
            .unwrap();
    }
    pub fn equals(&self, other: impl MiraiEventTrait<B>) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(
                &self.instance,
                "equals",
                &[InvocationArg::try_from(other.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_bot(&self) -> Bot<B> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getBot", InvocationArg::empty())
            .unwrap();
        Bot::try_from_instance(instance).unwrap()
    }
    pub fn get_event_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getEventId", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    // TODO: 以下一些函数会返回空值，需要做判断。
    pub fn get_from_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getFromId", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_from_nick(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getFromNick", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_group(&self) -> Group<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getGroup", InvocationArg::empty())
            .unwrap();
        Group::try_from_instance(instance).unwrap()
    }
    pub fn get_group_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getGroupId", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_group_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getGroupName", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_invitor(&self) -> NormalMember<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getInvitor", InvocationArg::empty())
            .unwrap();
        NormalMember::try_from_instance(instance).unwrap()
    }
    pub fn get_invitor_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getInvitorId", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_message(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getMessage", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn ignore(&self, blacklist: bool) {
        let jvm = Jvm::attach_thread().unwrap();
        let blacklist = InvocationArg::try_from(blacklist)
            .unwrap()
            .into_primitive()
            .unwrap();
        let _ = jvm.invoke(&self.instance, "ignore", &[blacklist]).unwrap();
    }
    pub fn reject(&self, blacklist: bool) {
        let jvm = Jvm::attach_thread().unwrap();
        let blacklist = InvocationArg::try_from(blacklist)
            .unwrap()
            .into_primitive()
            .unwrap();
        let _ = jvm.invoke(&self.instance, "reject", &[blacklist]).unwrap();
    }
}
impl<B: BotBackend> Display for MemberJoinRequestEvent<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "toString", InvocationArg::empty())
            .unwrap();
        f.write_str(jvm.to_rust::<String>(instance).unwrap().as_str())
    }
}
impl<B: BotBackend> MessageHashCodeTrait for MemberJoinRequestEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for MemberJoinRequestEvent<B> {}
