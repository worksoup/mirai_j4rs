use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BaseGroupMemberInfoChangeEventTrait, BotEventTrait};

#[mj_event]
pub struct BotInvitedJoinGroupRequestEvent {
    instance: Instance,
}
impl BotEventTrait for BotInvitedJoinGroupRequestEvent {}
impl BaseGroupMemberInfoChangeEventTrait for BotInvitedJoinGroupRequestEvent {}
