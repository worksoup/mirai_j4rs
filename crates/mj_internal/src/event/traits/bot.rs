use crate::contact::{Bot, ContactTrait, UserTrait};
use crate::event::{CancellableEventTrait, GroupMemberInfoChangeEventTrait, MiraiEventTrait};
use j4rs::Jvm;
use mj_base::env::FromInstance;

pub trait BotEventTrait
where
    Self: MiraiEventTrait,
{
    fn get_bot(&self) -> Bot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&self.get_instance(), "getBot", &[]).unwrap();
        Bot::from_instance(bot)
    }
}

pub trait BotActiveEventTrait: BotEventTrait {}
pub trait BotLeaveEventTrait: BotEventTrait + GroupMemberInfoChangeEventTrait {}
pub trait BotOfflineEventTrait: BotEventTrait {}
pub trait BotPassiveEventTrait: BotEventTrait {}
pub trait BaseGroupMemberInfoChangeEventTrait: BotEventTrait {
    fn get_group_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "getGroupId", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
}
pub trait FriendInfoChangeEventTrait: BotEventTrait {}
pub trait OtherClientEventTrait: BotEventTrait {}
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
