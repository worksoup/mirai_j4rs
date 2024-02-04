use crate::event::event_trait::{BotEventTrait, BotOfflineEventTrait, MiraiEventTrait};
use j4rs::{Instance, Jvm};
use mj_base::env::FromInstance;
use mj_macro::{java_type, AsInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotOnlineEvent")]
pub struct BotOnlineEvent {
    instance: Instance,
}

impl BotOnlineEvent {}

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
    //     TODO: EventKt
    // }
}

impl BotEventTrait for BotOnlineEvent {}

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
