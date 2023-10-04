use crate::{
    contact::{
        bot::Bot,
        contact_trait::{ContactTrait, UserOrBotTrait},
        Friend, NormalMember,
    },
    env::GetEnvTrait,
    message::message_trait::MessageHashCodeTrait,
};
use contact_derive::GetInstanceDerive;
use j4rs::{Instance, InvocationArg, Jvm};
use crate::contact::Stranger;
use crate::env::FromInstance;

pub trait Nudge: GetEnvTrait + MessageHashCodeTrait + FromInstance {
    type UserOrBot: UserOrBotTrait;
    fn get_target(&self) -> Self::UserOrBot {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "getTarget", &[]).unwrap();
        Self::UserOrBot::from_instance(instance)
    }
    // TODO: 该函数不符合 Mirai 定义的位置。到时候用 rust 标准库里的特征看看能不能实现一下。
    fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "toString", &[]).unwrap();
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
                &self.get_instance(),
                "sendTo",
                &[InvocationArg::try_from(receiver.get_instance()).unwrap()],
            )
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}

#[derive(GetInstanceDerive)]
pub struct BotNudge {
    pub(crate) instance: Instance,
}

impl Nudge for BotNudge {
    type UserOrBot = Bot;
}

impl FromInstance for BotNudge {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl MessageHashCodeTrait for BotNudge {}

#[derive(GetInstanceDerive)]
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

#[derive(GetInstanceDerive)]
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

#[derive(GetInstanceDerive)]
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