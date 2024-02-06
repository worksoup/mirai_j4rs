use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::BotEventTrait;

#[mj_event]
pub struct NudgeEvent {
    instance: Instance,
}
impl BotEventTrait for NudgeEvent {}
