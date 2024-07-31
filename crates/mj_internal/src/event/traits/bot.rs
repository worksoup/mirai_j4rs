use j4rs::InvocationArg;

use crate::{
    contact::{Bot, ContactTrait, Member, NormalMember, UserTrait},
    event::{
        CancellableEventTrait, GroupAwareMessageTrait, MiraiEventTrait, UserMessageEventTrait,
    },
    utils::backend::BotBackend,
};
use mj_helper_macro::java_fn;

pub trait BotEventTrait<B: BotBackend>
where
    Self: MiraiEventTrait<B>,
{
    #[java_fn]
    fn get_bot(&self) -> Bot<B> {}
}

pub trait BotActiveEventTrait<B: BotBackend>: BotEventTrait<B> {}
pub trait BotPassiveEventTrait<B: BotBackend>: BotEventTrait<B> {}
pub trait BaseGroupMemberInfoChangeEventTrait<B: BotBackend>: BotEventTrait<B> {
    #[java_fn]
    fn get_group_id(&self) -> i64 {}
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
