use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::BotEventTrait;

#[mj_event]
pub struct NudgeEvent {
    instance: Instance,
}
impl BotEventTrait for NudgeEvent {}
