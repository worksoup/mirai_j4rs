use crate::{
    contact::{Bot, ContactTrait, Friend, NormalMember, Stranger, UserOrBotTrait},
    message::message_trait::MessageHashCodeTrait,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{AsInstanceTrait, FromInstance, GetInstanceTrait};
use mj_macro::{AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

pub trait Nudge: GetInstanceTrait + MessageHashCodeTrait + FromInstance + AsInstanceTrait {
    type UserOrBot: UserOrBotTrait;
    fn get_target(&self) -> Self::UserOrBot {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(self.as_instance(), "getTarget", &[]).unwrap();
        Self::UserOrBot::from_instance(instance)
    }
    // TODO: 该函数不符合 Mirai 定义的位置。到时候用 rust 标准库里的特征看看能不能实现一下。
    fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(self.as_instance(), "toString", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    // TODO: 该函数不符合 Mirai 定义的位置。
    fn equals(&self) -> bool {
        todo!("低优先级。")
    }
    fn send_to(&self, receiver: impl ContactTrait) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(
                self.as_instance(),
                "sendTo",
                &[InvocationArg::try_from(receiver.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
pub struct BotNudge {
    instance: Instance,
}

impl Nudge for BotNudge {
    type UserOrBot = Bot;
}

impl MessageHashCodeTrait for BotNudge {}

#[derive(GetInstanceDerive, AsInstanceDerive)]
pub struct FriendNudge {
    pub(crate) instance: Instance,
}

impl FriendNudge {
    pub fn new(friend: Friend) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .create_instance(
                "",
                &[InvocationArg::try_from(friend.get_instance()).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
}

impl FromInstance for FriendNudge {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl Nudge for FriendNudge {
    type UserOrBot = Friend;
}

impl MessageHashCodeTrait for FriendNudge {}

#[derive(GetInstanceDerive, AsInstanceDerive)]
pub struct NormalMemberNudge {
    pub(crate) instance: Instance,
}

impl FromInstance for NormalMemberNudge {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl Nudge for NormalMemberNudge {
    type UserOrBot = NormalMember;
}

impl MessageHashCodeTrait for NormalMemberNudge {}

#[derive(GetInstanceDerive, AsInstanceDerive)]
pub struct StrangerNudge {
    pub(crate) instance: Instance,
}

impl FromInstance for StrangerNudge {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl Nudge for StrangerNudge {
    type UserOrBot = Stranger;
}

impl MessageHashCodeTrait for StrangerNudge {}
