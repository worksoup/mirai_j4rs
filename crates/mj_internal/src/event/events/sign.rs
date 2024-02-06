use crate::event::BotEventTrait;
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct SignEvent {
    instance: Instance,
}
impl BotEventTrait for SignEvent {}
