use crate::contact::{ContactOrBotTrait, ContactTrait, SendMessageSupportedTrait};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.contact.OtherClient")]
pub struct OtherClient {
    instance: Instance,
}

impl ContactOrBotTrait for OtherClient {}

impl ContactTrait for OtherClient {}

impl SendMessageSupportedTrait for OtherClient {}
