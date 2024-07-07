use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::BotEventTrait;

#[mj_event]
pub struct BotNickChangedEvent {
    instance: Instance,
}
impl BotEventTrait for BotNickChangedEvent {}
