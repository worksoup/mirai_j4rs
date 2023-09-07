use contact_derive::GetInstanceDerive;
use j4rs::{Instance, Jvm};

use crate::{contact::bot::Bot, env::GetClassTypeTrait};

use super::event_trait::MiraiEventTrait;

pub trait BotEventTrait
    where
        Self: MiraiEventTrait,
{
    fn get_bot(&self) -> Bot;
}

#[derive(GetInstanceDerive)]
pub struct BotOnlineEvent {
    instance: Instance,
}

impl GetClassTypeTrait for BotOnlineEvent {
    fn get_class_type() -> Instance {
        todo!()
    }
}

impl MiraiEventTrait for BotOnlineEvent {
    fn from_instance(instance: Instance) -> Self {
        todo!()
    }
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

    fn broadcast(&self) {
        todo!()
    }
}

impl BotEventTrait for BotOnlineEvent {
    fn get_bot(&self) -> Bot {
        todo!()
    }
}

pub trait BotOfflineEventTrait {}

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

