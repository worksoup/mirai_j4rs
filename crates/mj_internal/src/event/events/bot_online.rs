use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotActiveEventTrait, BotEventTrait};

#[mj_event]
pub struct BotOnlineEvent {
    instance: Instance,
}

impl BotOnlineEvent {}

impl BotEventTrait for BotOnlineEvent {}

impl BotActiveEventTrait for BotOnlineEvent {}
