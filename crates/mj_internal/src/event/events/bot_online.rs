use crate::event::{BotActiveEventTrait, BotEventTrait, MiraiEventTrait};
use j4rs::{Instance, Jvm};
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.event.events.BotOnlineEvent")]
pub struct BotOnlineEvent {
    instance: Instance,
}

impl BotOnlineEvent {}

impl BotEventTrait for BotOnlineEvent {}

impl BotActiveEventTrait for BotOnlineEvent {}

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
