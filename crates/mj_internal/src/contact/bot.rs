use crate::utils::backend::{BotBackend, Mirai};
use crate::{
    contact::{
        contact_trait::{ContactOrBotTrait, NudgeSupportedTrait, UserOrBotTrait},
        Friend, Group, OtherClient, Stranger,
    },
    error::MiraiRsError,
    event::EventChannel,
    utils::{
        contact::{friend_group::FriendGroups, ContactList},
        other::enums::AvatarSpec,
        BotConfiguration, MiraiLogger,
    },
};
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{
    utils::{instance_is_null, java_iter_to_rust_vec},
    FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait, TryFromInstanceTrait,
};
use mj_helper_macro::{error_msg_suppressor, java_fn, mj_all};

#[mj_all("Bot")]
pub struct Bot<Backend: BotBackend> {
    instance: Instance,
    backend: Backend,
}
impl<B: BotBackend> Clone for Bot<B> {
    fn clone(&self) -> Self {
        Bot {
            instance: self.get_instance().unwrap(),
            backend: self.backend.clone(),
        }
    }
}
impl Bot<Mirai> {
    #[java_fn]
    pub fn login(&self) {}
    #[java_fn]
    pub fn get_configuration(&self) -> BotConfiguration {}
}
impl<B: BotBackend> Bot<B> {
    pub fn get_bots() -> Vec<Self> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                (<Self as GetClassTypeTrait>::get_type_name().to_string() + "$Companion").as_str(),
                "getInstances",
                InvocationArg::empty(),
            )
            .unwrap();
        let iter = jvm
            .invoke(&instance, "iterator", InvocationArg::empty())
            .unwrap();
        java_iter_to_rust_vec(&jvm, iter)
    }

    pub fn find_bot(id: i64) -> Option<Self> {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm
            .invoke_static(
                (<Self as GetClassTypeTrait>::get_type_name().to_string() + "$Companion").as_str(),
                "findInstance",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&bot) {
            Some(Bot {
                instance: bot,
                backend: B::default(),
            })
        } else {
            None
        }
    }
    #[java_fn]
    pub fn close(self) {}
    pub fn close_and_join(self, _err: MiraiRsError) {
        // TODO
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "closeAndJoin", InvocationArg::empty())
            .unwrap();
    }
    #[java_fn]
    pub fn get_as_friend(&self) -> Friend<B> {}
    #[java_fn]
    pub fn get_as_stranger(&self) -> Stranger<B> {}
    #[java_fn]
    pub fn get_event_channel(&self) -> EventChannel<B> {}
    #[java_fn]
    pub fn get_friend(&self, id: i64) -> Option<Friend<B>> {
        let instance = error_msg_suppressor!("instance");
        if !instance_is_null(&instance) {
            Some(Friend::try_from_instance(instance).unwrap())
        } else {
            None
        }
    }
    #[java_fn]
    pub fn get_friend_groups(&self) -> FriendGroups<B> {}
    #[java_fn]
    pub fn get_friends(&self) -> ContactList<B, Friend<B>> {}
    #[java_fn]
    pub fn get_group(&self, id: i64) -> Option<Group<B>> {
        let instance = error_msg_suppressor!("instance");
        if !instance_is_null(&instance) {
            Some(Group::from_instance(instance))
        } else {
            None
        }
    }
    #[java_fn]
    pub fn get_groups(&self) -> ContactList<B, Group<B>> {}
    pub fn get_logger() -> MiraiLogger {
        todo!("get logger")
    }
    #[java_fn]
    pub fn get_other_clients(&self) -> ContactList<B, OtherClient<B>> {}
    #[java_fn]
    pub fn get_stranger(&self, id: i64) -> Option<Stranger<B>> {
        let instance = error_msg_suppressor!("instance");
        if !instance_is_null(&instance) {
            Some(Stranger::from_instance(instance))
        } else {
            None
        }
    }
    #[java_fn]
    pub fn get_strangers(&self) -> ContactList<B, Stranger<B>> {}
    #[java_fn]
    pub fn is_online(&self) -> bool {}
    #[java_fn]
    pub fn join(&self) {}
}

impl<B: BotBackend> ContactOrBotTrait<B> for Bot<B> {
    fn get_avatar_url(&self, size: Option<AvatarSpec>) -> String {
        let size: i32 = if let Some(size) = size {
            size.into()
        } else {
            AvatarSpec::XL.into()
        };
        // 这里 Mirai 源码中应该是 http 而不是 https.
        let mut url = "https://q.qlogo.cn/g?b=qq&nk=".to_string();
        url.push_str(self.get_id().to_string().as_str());
        url.push_str("&s=");
        url.push_str(size.to_string().as_str());
        url
    }
}

impl<B: BotBackend> UserOrBotTrait<B> for Bot<B> {}

impl<B: BotBackend> NudgeSupportedTrait<B> for Bot<B> {}
