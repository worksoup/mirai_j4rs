use crate::contact::bot::Bot;
use crate::message::data::message_chain::MessageChain;
use j4rs::Jvm;
use mj_base::env::{FromInstance, GetClassTypeTrait, GetEnvTrait};

pub trait MiraiEventTrait
where
    Self: GetEnvTrait + GetClassTypeTrait + FromInstance,
{
    fn cancel(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.get_instance(), "cancel", &[]).unwrap();
    }
    fn intercept(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.get_instance(), "intercept", &[]).unwrap();
    }
    fn is_canceled(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.get_instance(), "isCanceled", &[]).unwrap())
            .unwrap()
    }
    fn is_intercepted(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.get_instance(), "isIntercepted", &[])
                .unwrap(),
        )
        .unwrap()
    }
    /// 广播一个事件。
    fn broadcast(&self) {
        todo!("参见 EventKt")
    }
}

pub trait BotEventTrait
where
    Self: MiraiEventTrait,
{
    fn get_bot(&self) -> Bot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&self.get_instance(), "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&bot, "getId", &[]).unwrap())
            .unwrap();
        Bot { instance: bot, id }
    }
}

pub trait BotOfflineEventTrait {}

pub trait MessageEventTrait
where
    Self: MiraiEventTrait,
{
    fn get_bot(&self) -> Bot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&self.get_instance(), "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&bot, "getId", &[]).unwrap())
            .unwrap();
        Bot { instance: bot, id }
    }
    fn get_message(&self) -> MessageChain {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "getMessage", &[]).unwrap();
        MessageChain { instance }
    }
    type UserItem;
    fn get_sender(&self) -> Self::UserItem;
    fn get_sender_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.get_instance(), "getSenderName", &[])
                .unwrap(),
        )
        .unwrap()
    }
    fn get_source(&self) -> () {
        todo!("net.mamoe.mirai.message.data.OnlineMessageSource.Incoming")
    }
    type ContactItem;
    fn get_subject(&self) -> Self::ContactItem;
    fn get_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.get_instance(), "getTime", &[]).unwrap())
            .unwrap()
    }
}

pub trait FriendInfoChangedEvent: BotEventTrait {}
