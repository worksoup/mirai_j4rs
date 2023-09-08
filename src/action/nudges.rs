use crate::contact::bot::Bot;
use crate::contact::contact_trait::{ContactTrait, UserOrBotTrait};
use crate::contact::{Friend, NormalMember};
use crate::env::GetEnvTrait;
use contact_derive::GetInstanceDerive;
use j4rs::{Instance, InvocationArg, Jvm};
use crate::message::message_trait::MessageHashCodeTrait;

pub trait Nudge: GetEnvTrait {
    type UserOrBot: UserOrBotTrait;
    fn internal_build_user_or_bot_from_instance(instance: Instance) -> Self::UserOrBot;
    fn get_target(&self) -> Self::UserOrBot {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "getTarget", &[]).unwrap();
        Self::internal_build_user_or_bot_from_instance(instance)
    }
    // TODO: 该函数不符合 Mirai 定义的位置。到时候用 rust 标准库里的特征看看能不能实现一下。
    fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(
                &self.get_instance(),
                "toString",
                &[],
            )
            .unwrap();
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

    fn internal_build_user_or_bot_from_instance(bot: Instance) -> Self::UserOrBot {
        let jvm = Jvm::attach_thread().unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&bot, "getId", &[]).unwrap())
            .unwrap();
        Bot { bot, id }
    }
}

impl MessageHashCodeTrait for BotNudge {}

#[derive(GetInstanceDerive)]
pub struct FriendNudge {
    pub(crate) instance: Instance,
}

impl Nudge for FriendNudge {
    type UserOrBot = Friend;

    fn internal_build_user_or_bot_from_instance(instance: Instance) -> Self::UserOrBot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        Self::UserOrBot { bot, instance, id }
    }
}

impl MessageHashCodeTrait for FriendNudge {}

#[derive(GetInstanceDerive)]
pub struct MemberNudge {
    pub(crate) instance: Instance,
}

impl Nudge for MemberNudge {
    type UserOrBot = NormalMember;

    fn internal_build_user_or_bot_from_instance(instance: Instance) -> Self::UserOrBot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        Self::UserOrBot { bot, instance, id }
    }
}

impl MessageHashCodeTrait for MemberNudge {}