use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupEventTrait, GroupMemberEventTrait, UserEventTrait,
};

#[mj_event]
pub struct MemberHonorChangeEvent {
    instance: Instance,
}
impl GroupMemberEventTrait for MemberHonorChangeEvent {}
impl BotPassiveEventTrait for MemberHonorChangeEvent {}

impl GroupEventTrait for MemberHonorChangeEvent {}
impl BotEventTrait for MemberHonorChangeEvent {}
impl UserEventTrait for MemberHonorChangeEvent {}
