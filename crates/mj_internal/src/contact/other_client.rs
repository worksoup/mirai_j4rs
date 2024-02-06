use j4rs::Instance;

use mj_macro::mj_all;

use crate::contact::{ContactOrBotTrait, ContactTrait, SendMessageSupportedTrait};

#[mj_all("contact.OtherClient")]
pub struct OtherClient {
    instance: Instance,
}

impl ContactOrBotTrait for OtherClient {}

impl ContactTrait for OtherClient {}

impl SendMessageSupportedTrait for OtherClient {}
