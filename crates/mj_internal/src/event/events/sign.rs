use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::BotEventTrait;

#[mj_event]
pub struct SignEvent {
    instance: Instance,
}
impl BotEventTrait for SignEvent {}
