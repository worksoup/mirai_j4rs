use j4rs::{InvocationArg, Jvm};

use jbuchong::TryFromInstanceTrait;

use crate::contact::{Bot, ContactTrait, Member, NormalMember, UserTrait};
use crate::event::{
    CancellableEventTrait, GroupAwareMessageTrait, MiraiEventTrait, UserMessageEventTrait,
};

pub trait BotEventTrait
where
    Self: MiraiEventTrait,
{
    fn get_bot(&self) -> Bot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm
            .invoke(&self.as_instance(), "getBot", InvocationArg::empty())
            .unwrap();
        Bot::try_from_instance(bot).unwrap()
    }
}

pub trait BotActiveEventTrait: BotEventTrait {}
pub trait BotPassiveEventTrait: BotEventTrait {}
pub trait BaseGroupMemberInfoChangeEventTrait: BotEventTrait {
    fn get_group_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.as_instance(), "getGroupId", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
}
pub trait FriendInfoChangeEventTrait: BotEventTrait {}
// TODO
pub trait MessageRecallTrait: BotEventTrait {}
// TODO
pub trait MessagePostSendEventTrait<T: ContactTrait>: BotEventTrait + BotActiveEventTrait {}
// TODO
pub trait UserMessagePostSendEventTrait<T: UserTrait>: MessagePostSendEventTrait<T> {}
// TODO
pub trait MessagePreSendEventTrait:
    BotEventTrait + BotActiveEventTrait + CancellableEventTrait
{
}
pub trait UserMessagePreSendEventTrait: MessagePreSendEventTrait {}
pub trait TempMessagePostSendEventTrait: UserMessagePostSendEventTrait<Member> {}
pub trait TempMessagePreSendEventTrait: UserMessagePreSendEventTrait {}
pub trait TempMessageEventTrait<Subject: ContactTrait>:
    GroupAwareMessageTrait<NormalMember, Subject> + UserMessageEventTrait<NormalMember, Subject>
{
}
