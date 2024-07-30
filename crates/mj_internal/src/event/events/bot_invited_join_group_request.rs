use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BaseGroupMemberInfoChangeEventTrait, BotEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct BotInvitedJoinGroupRequestEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> BotEventTrait<B> for BotInvitedJoinGroupRequestEvent<B> {}
impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for BotInvitedJoinGroupRequestEvent<B> {}
