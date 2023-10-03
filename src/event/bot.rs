use crate::{
    env::FromInstance,
    event::event_trait::{BotEventTrait, BotOfflineEventTrait, MiraiEventTrait},
};
use contact_derive::{GetClassTypeDerive, GetInstanceDerive};
use j4rs::{Instance, Jvm};

#[derive(GetInstanceDerive, GetClassTypeDerive)]
pub struct BotOnlineEvent {
    instance: Instance,
}

impl BotOnlineEvent {
    fn get_class_name() -> String {
        "net.mamoe.mirai.event.events.BotOnlineEvent".to_string()
    }
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
