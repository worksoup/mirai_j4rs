use j4rs::{InvocationArg, Jvm};

use jbuchong::TryFromInstanceTrait;

use crate::contact::{Bot, ContactTrait, Member, NormalMember, UserTrait};
use crate::event::{
    CancellableEventTrait, GroupAwareMessageTrait, MiraiEventTrait, UserMessageEventTrait,
};
use crate::utils::backend::BotBackend;

pub trait BotEventTrait<B: BotBackend>
where
    Self: MiraiEventTrait<B>,
{
    fn get_bot(&self) -> Bot<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm
            .invoke(self.as_instance(), "getBot", InvocationArg::empty())
            .unwrap();
        Bot::try_from_instance(bot).unwrap()
    }
}

pub trait BotActiveEventTrait<B: BotBackend>: BotEventTrait<B> {}
pub trait BotPassiveEventTrait<B: BotBackend>: BotEventTrait<B> {}
pub trait BaseGroupMemberInfoChangeEventTrait<B: BotBackend>: BotEventTrait<B> {
    fn get_group_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(self.as_instance(), "getGroupId", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}
pub trait FriendInfoChangeEventTrait<B: BotBackend>: BotEventTrait<B> {}
// TODO
pub trait MessageRecallTrait<B: BotBackend>: BotEventTrait<B> {}
// TODO
pub trait MessagePostSendEventTrait<B: BotBackend, T: ContactTrait<B>>:
    BotEventTrait<B> + BotActiveEventTrait<B>
{
}
// TODO
pub trait UserMessagePostSendEventTrait<B: BotBackend, T: UserTrait<B>>:
    MessagePostSendEventTrait<B, T>
{
}
// TODO
pub trait MessagePreSendEventTrait<B: BotBackend>:
    BotEventTrait<B> + BotActiveEventTrait<B> + CancellableEventTrait<B>
{
}
pub trait UserMessagePreSendEventTrait<B: BotBackend>: MessagePreSendEventTrait<B> {}
pub trait TempMessagePostSendEventTrait<B: BotBackend>:
    UserMessagePostSendEventTrait<B, Member<B>>
{
}
pub trait TempMessagePreSendEventTrait<B: BotBackend>: UserMessagePreSendEventTrait<B> {}
pub trait TempMessageEventTrait<B: BotBackend, Subject: ContactTrait<B>>:
    GroupAwareMessageTrait<B, NormalMember<B>, Subject>
    + UserMessageEventTrait<B, NormalMember<B>, Subject>
{
}
