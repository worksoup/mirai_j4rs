use crate::contact::{ContactOrBotTrait, ContactTrait, SendMessageSupportedTrait};
use j4rs::Instance;
use mj_base::env::FromInstance;
use mj_macro::{AsInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive)]
pub struct OtherClient {
    instance: Instance,
}

impl FromInstance for OtherClient {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl ContactOrBotTrait for OtherClient {}

impl ContactTrait for OtherClient {}

impl SendMessageSupportedTrait for OtherClient {}
