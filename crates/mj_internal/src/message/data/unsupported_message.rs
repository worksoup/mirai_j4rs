use j4rs::Instance;

use jbuchong::{AsInstanceDerive, TryFromInstanceDerive, GetInstanceDerive};

use crate::message::message_trait::{MessageContentTrait, MessageTrait, SingleMessageTrait};

#[derive(AsInstanceDerive, GetInstanceDerive, TryFromInstanceDerive)]
pub struct UnsupportedMessage {
    instance: Instance,
}

impl MessageTrait for UnsupportedMessage {}

impl SingleMessageTrait for UnsupportedMessage {}

impl MessageContentTrait for UnsupportedMessage {}
