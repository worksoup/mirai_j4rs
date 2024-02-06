use crate::event::{BaseGroupMemberInfoChangeEventTrait, BotEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BotInvitedJoinGroupRequestEvent {
    instance: Instance,
}
impl BotEventTrait for BotInvitedJoinGroupRequestEvent {}
impl BaseGroupMemberInfoChangeEventTrait for BotInvitedJoinGroupRequestEvent {}
