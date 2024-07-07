use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_macro::{AsInstanceDerive, TryFromInstanceDerive, GetInstanceDerive};

use crate::message::message_trait::{MessageContentTrait, MessageTrait, SingleMessageTrait};

#[derive(AsInstanceDerive, GetInstanceDerive, TryFromInstanceDerive)]
pub struct UnsupportedMessage {
    instance: Instance,
}

impl MessageTrait for UnsupportedMessage {}

impl SingleMessageTrait for UnsupportedMessage {}

impl MessageContentTrait for UnsupportedMessage {}
