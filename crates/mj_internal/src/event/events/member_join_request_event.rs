use j4rs::{Instance, InvocationArg, Jvm};
use mj_helper_macro::{java_fn, mj_event};
use std::fmt::{Display, Formatter};

use crate::contact::{Bot, Group, NormalMember};
use crate::event::{BotEventTrait, MiraiEventTrait};
use crate::message::MessageHashCodeTrait;
use crate::utils::backend::BotBackend;
use crate::utils::data_wrapper::{DataWrapper, PrimitiveConvert};

// TODO: BaseGroupMemberInfoChangeEvent
#[mj_event]
pub struct MemberJoinRequestEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MemberJoinRequestEvent<B> {
    // TODO: check if the method name is wrong.
    pub fn accept(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm
            .invoke(&self.instance, "getBot", InvocationArg::empty())
            .unwrap();
    }
    #[java_fn("equals")]
    fn equals_(&self, other: Instance) -> bool {}
    pub fn equals(&self, other: impl MiraiEventTrait<B>) -> bool {
        self.equals_(other.get_instance().unwrap())
    }
    #[java_fn]
    pub fn get_bot(&self) -> Bot<B> {}
    #[java_fn]
    pub fn get_event_id(&self) -> i64 {}
    // TODO: 以下一些函数会返回空值，需要做判断。
    #[java_fn]
    pub fn get_from_id(&self) -> i64 {}
    #[java_fn]
    pub fn get_from_nick(&self) -> String {}
    #[java_fn]
    pub fn get_group(&self) -> Group<B> {}
    #[java_fn]
    pub fn get_group_id(&self) -> String {}
    #[java_fn]
    pub fn get_group_name(&self) -> String {}
    #[java_fn]
    pub fn get_invitor(&self) -> NormalMember<B> {}
    #[java_fn]
    pub fn get_invitor_id(&self) -> i64 {}
    #[java_fn]
    pub fn get_message(&self) -> String {}
    #[java_fn]
    pub fn ignore(&self, blacklist: DataWrapper<bool, PrimitiveConvert>) {}
    #[java_fn]
    pub fn reject(&self, blacklist: DataWrapper<bool, PrimitiveConvert>) {}
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
