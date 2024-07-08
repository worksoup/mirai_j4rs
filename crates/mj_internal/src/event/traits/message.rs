use j4rs::{InvocationArg, Jvm};

use jbuchong::{FromInstanceTrait, TryFromInstanceTrait};

use crate::contact::{Bot, ContactTrait};
use crate::event::{BotPassiveEventTrait, MiraiEventTrait, OtherClientEventTrait};
use crate::message::data::MessageChain;

pub trait MessageEventTrait<Sender: ContactTrait, Subject: ContactTrait>
where
    Self: MiraiEventTrait + BotPassiveEventTrait + OtherClientEventTrait,
{
    fn get_bot(&self) -> Bot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm
            .invoke(&self.as_instance(), "getBot", InvocationArg::empty())
            .unwrap();
        Bot::try_from_instance(bot).unwrap()
    }
    fn get_message(&self) -> MessageChain {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.as_instance(), "getMessage", InvocationArg::empty())
            .unwrap();
        MessageChain::from_instance(instance)
    }
    fn get_sender(&self) -> Sender {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.as_instance(), "getSender", InvocationArg::empty())
            .unwrap();
        Sender::try_from_instance(instance).unwrap()
    }
    fn get_sender_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.as_instance(), "getSenderName", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    fn get_source(&self) -> () {
        todo!("message.data.OnlineMessageSource.Incoming")
    }
    fn get_subject(&self) -> Subject {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.as_instance(), "getSubject", InvocationArg::empty())
            .unwrap();
        Subject::try_from_instance(instance).unwrap()
    }
    fn get_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.as_instance(), "getTime", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
}

// TODO
pub trait MessageSyncEventTrait<Sender: ContactTrait, Subject: ContactTrait>:
    MessageEventTrait<Sender, Subject> + OtherClientEventTrait
{
}

pub trait GroupAwareMessageTrait<Sender: ContactTrait, Subject: ContactTrait>:
    MessageEventTrait<Sender, Subject>
{
}

pub trait UserMessageEventTrait<Sender: ContactTrait, Subject: ContactTrait>:
    MessageEventTrait<Sender, Subject>
{
}
