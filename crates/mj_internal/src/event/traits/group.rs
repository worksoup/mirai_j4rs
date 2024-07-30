use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

pub trait GroupEventTrait<B: BotBackend>: BotEventTrait<B> {}
pub trait GroupMemberEventTrait<B: BotBackend>: GroupEventTrait<B> + UserEventTrait<B> {}
pub trait GroupMemberInfoChangeEventTrait<B: BotBackend>:
    BotEventTrait<B> + GroupEventTrait<B> + BaseGroupMemberInfoChangeEventTrait<B>
{
}
pub trait GroupOperableEventTrait<B: BotBackend>: GroupEventTrait<B> {}
pub trait GroupSettingsChangeEventTrait<B: BotBackend, T>:
    GroupEventTrait<B> + BotPassiveEventTrait<B> + BroadcastControllableTrait<B>
{
}
