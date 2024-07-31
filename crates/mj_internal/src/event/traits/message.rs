use j4rs::InvocationArg;

use crate::{
    contact::{Bot, ContactTrait},
    event::{BotPassiveEventTrait, MiraiEventTrait, OtherClientEventTrait},
    message::data::MessageChain,
    utils::backend::BotBackend,
};
use mj_helper_macro::java_fn;

pub trait MessageEventTrait<B: BotBackend, Sender: ContactTrait<B>, Subject: ContactTrait<B>>
where
    Self: MiraiEventTrait<B> + BotPassiveEventTrait<B> + OtherClientEventTrait<B>,
{
    #[java_fn]
    fn get_bot(&self) -> Bot<B> {}
    #[java_fn]
    fn get_message(&self) -> MessageChain<B> {}
    #[java_fn]
    fn get_sender(&self) -> Sender {}
    #[java_fn]
    fn get_sender_name(&self) -> String {}
    fn get_source(&self) {
        todo!("message.data.OnlineMessageSource.Incoming")
    }
    #[java_fn]
    fn get_subject(&self) -> Subject {}
    #[java_fn]
    fn get_time(&self) -> i64 {}
}

// TODO
pub trait MessageSyncEventTrait<B: BotBackend, Sender: ContactTrait<B>, Subject: ContactTrait<B>>:
    MessageEventTrait<B, Sender, Subject> + OtherClientEventTrait<B>
{
}

pub trait GroupAwareMessageTrait<B: BotBackend, Sender: ContactTrait<B>, Subject: ContactTrait<B>>:
    MessageEventTrait<B, Sender, Subject>
{
}

pub trait UserMessageEventTrait<B: BotBackend, Sender: ContactTrait<B>, Subject: ContactTrait<B>>:
    MessageEventTrait<B, Sender, Subject>
{
}
