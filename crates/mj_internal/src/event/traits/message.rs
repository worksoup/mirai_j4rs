use crate::contact::{Bot, ContactTrait};
use crate::event::{BotPassiveEventTrait, MiraiEventTrait, OtherClientEventTrait};
use crate::message::data::MessageChain;
use j4rs::Jvm;
use mj_base::env::FromInstance;

pub trait MessageEventTrait<Sender: ContactTrait, Subject: ContactTrait>
where
    Self: MiraiEventTrait + BotPassiveEventTrait + OtherClientEventTrait,
{
    fn get_bot(&self) -> Bot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&self.as_instance(), "getBot", &[]).unwrap();
        Bot::from_instance(bot)
    }
    fn get_message(&self) -> MessageChain {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.as_instance(), "getMessage", &[]).unwrap();
        MessageChain { instance }
    }
    fn get_sender(&self) -> Sender {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.as_instance(), "getSender", &[]).unwrap();
        Sender::from_instance(instance)
    }
    fn get_sender_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.as_instance(), "getSenderName", &[])
                .unwrap(),
        )
        .unwrap()
    }
    fn get_source(&self) -> () {
        todo!("net.mamoe.mirai.message.data.OnlineMessageSource.Incoming")
    }
    fn get_subject(&self) -> Subject {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.as_instance(), "getSubject", &[]).unwrap();
        Subject::from_instance(instance)
    }
    fn get_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.as_instance(), "getTime", &[]).unwrap())
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
