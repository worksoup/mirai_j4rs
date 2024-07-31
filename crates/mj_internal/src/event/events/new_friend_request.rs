use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{utils::instance_is_null, TryFromInstanceTrait};
use mj_helper_macro::{error_msg_suppressor, java_fn, mj_event};
use std::fmt::{Display, Formatter};

use crate::contact::Group;
use crate::event::{BotEventTrait, FriendInfoChangeEventTrait};
use crate::message::MessageHashCodeTrait;
use crate::utils::backend::BotBackend;
use crate::utils::data_wrapper::{DataWrapper, PrimitiveConvert};

#[mj_event]
pub struct NewFriendRequestEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> NewFriendRequestEvent<B> {
    #[java_fn]
    pub fn accept(&self) {}
    #[java_fn]
    pub fn reject(&self, black_list: DataWrapper<bool, PrimitiveConvert>) {}
    #[java_fn]
    pub fn get_event_id(&self) -> i64 {}
    #[java_fn]
    pub fn get_from_group(&self) -> Option<Group<B>> {
        let group = error_msg_suppressor!("instance");
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
    #[java_fn]
    pub fn get_from_id(&self) -> i64 {}
    #[java_fn]
    pub fn get_from_nick(&self) -> String {}
    // TODO: check the method name;
    #[java_fn("getMessage")]
    pub fn get_from_message(&self) -> String {}
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
