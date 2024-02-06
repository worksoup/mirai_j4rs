use crate::event::{BotActiveEventTrait, BotEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BotOnlineEvent {
    instance: Instance,
}

impl BotOnlineEvent {}

impl BotEventTrait for BotOnlineEvent {}

impl BotActiveEventTrait for BotOnlineEvent {}
