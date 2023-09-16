use contact_derive::{GetClassTypeDerive, GetInstanceDerive};
use j4rs::{Instance, Jvm};

use crate::contact::bot::Bot;
use crate::env::FromInstance;
use crate::event::event_trait::{BotEventTrait, BotOfflineEventTrait};

use super::event_trait::MiraiEventTrait;

#[derive(GetInstanceDerive, GetClassTypeDerive)]
pub struct BotOnlineEvent {
    instance: Instance,
}

impl BotOnlineEvent {
    fn get_class_name() -> String { "net.mamoe.mirai.event.events.BotOnlineEvent".to_string() }
}

impl FromInstance for BotOnlineEvent {
    fn from_instance(instance: Instance) -> Self {
        BotOnlineEvent { instance }
    }
}

impl MiraiEventTrait for BotOnlineEvent {
    fn cancel(&self) {
        let _ = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "cancel", &[]);
    }

    fn intercept(&self) {
        let _ = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "intercept", &[]);
    }

    fn is_canceled(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "isCanceled", &[])
                    .unwrap(),
            )
            .unwrap()
    }

    fn is_intercepted(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "isIntercepted", &[])
                    .unwrap(),
            )
            .unwrap()
    }

    // fn broadcast(&self) {
    //     TODO: 这个函数哪来的？为什么在最初的版本中？
    // }
}

impl BotEventTrait for BotOnlineEvent {
    fn get_bot(&self) -> Bot {
        let jvm = Jvm::attach_thread().unwrap();
        let bot =
            jvm.invoke(&self.instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&bot, "getId", &[]).unwrap())
            .unwrap();
        Bot { bot, id }
    }
}

pub struct Active {}

pub struct Force {}

pub struct Dropped {}

pub struct RequireReconnect {}

pub enum BotOfflineEvent {
    Active(Active),
    Force(Force),
    Dropped(Dropped),
    RequireReconnect(RequireReconnect),
}

impl BotOfflineEventTrait for BotOfflineEvent {}

pub struct BotReloginEvent {}

pub struct BotAvatarChangedEvent {}

pub struct BotNickChangedEvent {}

pub struct NudgeEvent {}

