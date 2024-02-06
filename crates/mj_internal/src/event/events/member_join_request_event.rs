use crate::contact::{Bot, Group, NormalMember};
use crate::event::{BotEventTrait, MiraiEventTrait};
use crate::message::MessageHashCodeTrait;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::FromInstanceTrait;
use mj_macro::mj_event;

// TODO: BaseGroupMemberInfoChangeEvent
#[mj_event]
pub struct MemberJoinRequestEvent {
    instance: Instance,
}

impl MemberJoinRequestEvent {
    pub fn accept(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "getBot", &[]).unwrap();
    }
    pub fn equals(&self, other: impl MiraiEventTrait) -> bool {
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
    pub fn get_bot(&self) -> Bot {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getBot", &[])
            .unwrap();
        Bot::from_instance(instance)
    }
    pub fn get_event_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getEventId", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    // TODO: 以下一些函数会返回空值，需要做判断。
    pub fn get_from_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getFromId", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_from_nick(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getFromNick", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_group(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getGroup", &[]).unwrap();
        Group::from_instance(instance)
    }
    pub fn get_group_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getGroupId", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_group_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getGroupName", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_invitor(&self) -> NormalMember {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getInvitor", &[]).unwrap();
        NormalMember::from_instance(instance)
    }
    pub fn get_invitor_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getInvitorId", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn get_message(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getMessage", &[]).unwrap();
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
    pub fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "toString", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
}
impl MessageHashCodeTrait for MemberJoinRequestEvent {}

impl BotEventTrait for MemberJoinRequestEvent {}
