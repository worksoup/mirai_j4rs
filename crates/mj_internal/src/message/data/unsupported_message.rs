use j4rs::Instance;

use mj_macro::{AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

use crate::message::message_trait::{MessageContentTrait, MessageTrait, SingleMessageTrait};

#[derive(AsInstanceDerive, GetInstanceDerive, FromInstanceDerive)]
pub struct UnsupportedMessage {
    instance: Instance,
}

impl MessageTrait for UnsupportedMessage {}

impl SingleMessageTrait for UnsupportedMessage {}

impl MessageContentTrait for UnsupportedMessage {}
