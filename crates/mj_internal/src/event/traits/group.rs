use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, UserEventTrait,
};

pub trait GroupEventTrait: BotEventTrait {}
pub trait GroupMemberEventTrait: GroupEventTrait + UserEventTrait {}
pub trait GroupMemberInfoChangeEventTrait:
    BotEventTrait + GroupEventTrait + BaseGroupMemberInfoChangeEventTrait
{
}
pub trait GroupOperableEventTrait: GroupEventTrait {}
pub trait GroupSettingsChangeEventTrait<T>:
    GroupEventTrait + BotPassiveEventTrait + BroadcastControllableTrait
{
}
