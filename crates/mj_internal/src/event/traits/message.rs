use j4rs::{InvocationArg, Jvm};

use jbuchong::{FromInstanceTrait, TryFromInstanceTrait};

use crate::contact::{Bot, ContactTrait};
use crate::event::{BotPassiveEventTrait, MiraiEventTrait, OtherClientEventTrait};
use crate::message::data::MessageChain;
use crate::utils::backend::BotBackend;

pub trait MessageEventTrait<B: BotBackend, Sender: ContactTrait<B>, Subject: ContactTrait<B>>
where
    Self: MiraiEventTrait<B> + BotPassiveEventTrait<B> + OtherClientEventTrait<B>,
{
    fn get_bot(&self) -> Bot<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm
            .invoke(self.as_instance(), "getBot", InvocationArg::empty())
            .unwrap();
        Bot::try_from_instance(bot).unwrap()
    }
    fn get_message(&self) -> MessageChain<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(self.as_instance(), "getMessage", InvocationArg::empty())
            .unwrap();
        MessageChain::from_instance(instance)
    }
    fn get_sender(&self) -> Sender {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(self.as_instance(), "getSender", InvocationArg::empty())
            .unwrap();
        Sender::try_from_instance(instance).unwrap()
    }
    fn get_sender_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(self.as_instance(), "getSenderName", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    fn get_source(&self) {
        todo!("message.data.OnlineMessageSource.Incoming")
    }
    fn get_subject(&self) -> Subject {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(self.as_instance(), "getSubject", InvocationArg::empty())
            .unwrap();
        Subject::try_from_instance(instance).unwrap()
    }
    fn get_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(self.as_instance(), "getTime", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
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
