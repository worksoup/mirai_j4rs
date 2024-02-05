use crate::utils::contact::ContactList;
use crate::utils::{BotConfiguration, MiraiLogger};
use crate::{
    contact::{
        contact_trait::{ContactOrBotTrait, NudgeSupportedTrait, UserOrBotTrait},
        Friend, Group, OtherClient, Stranger,
    },
    error::MiraiRsError,
    event::EventChannel,
    message::action::BotNudge,
    utils::{contact::friend_group::FriendGroups, other::enums::AvatarSpec},
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::utils::java_iter_to_rust_vec;
use mj_base::{
    env::{FromInstance, GetInstanceTrait},
    utils::instance_is_null,
};
use mj_macro::{AsInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive)]
pub struct Bot {
    _jvm: Jvm,
    instance: Instance,
    id: i64,
}
impl Clone for Bot {
    fn clone(&self) -> Self {
        Bot {
            _jvm: Jvm::attach_thread().unwrap(),
            instance: self.get_instance(),
            ..*self
        }
    }
}
impl FromInstance for Bot {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let id = Jvm::attach_thread()
            .unwrap()
            .chain(&instance)
            .unwrap()
            .invoke("getId", &[])
            .unwrap()
            .to_rust()
            .unwrap();
        Bot {
            _jvm: jvm,
            instance,
            id,
        }
    }
}

impl Bot {
    pub fn get_bots() -> Vec<Bot> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static("net.mamoe.mirai.Bot$Companion", "getInstances", &[])
            .unwrap();
        let iter = jvm.invoke(&instance, "iterator", &[]).unwrap();
        java_iter_to_rust_vec(&jvm, iter)
    }

    pub fn find_bot(id: i64) -> Option<Bot> {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm
            .invoke_static(
                "net.mamoe.mirai.Bot$Companion",
                "findInstance",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&bot) {
            Some(Bot {
                _jvm: jvm,
                instance: bot,
                id,
            })
        } else {
            None
        }
    }
    pub fn close(self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "close", &[])
            .unwrap();
    }
    pub fn close_and_join(self, err: MiraiRsError) {
        // TODO
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "closeAndJoin", &[])
            .unwrap();
    }
    pub fn get_as_friend(&self) -> Friend {
        Friend::from_bot(self)
    }
    pub fn get_as_stranger(&self) -> Stranger {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getAsStranger", &[])
            .unwrap();
        Stranger::from_instance(instance)
    }
    pub fn get_configuration(&self) -> BotConfiguration {
        let bot_configuration = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getConfiguration", &[])
            .unwrap();
        BotConfiguration::from_instance(bot_configuration)
    }
    pub fn get_event_channel(&self) -> EventChannel {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getEventChannel", &[])
            .unwrap();
        EventChannel { jvm, instance }
    }
    pub fn get_friend(&self, id: i64) -> Option<Friend> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "getFriend",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Some(Friend::from_instance(instance))
        } else {
            None
        }
    }
    pub fn get_friend_groups(&self) -> FriendGroups {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getFriendGroups", &[]).unwrap();
        FriendGroups { instance }
    }
    pub fn get_friends(&self) -> ContactList<Friend> {
        ContactList::from_instance(
            Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "getFriends", &[])
                .unwrap(),
        )
    }
    pub fn get_group(&self, id: i64) -> Option<Group> {
        Group::new(self, id)
    }
    pub fn get_groups(&self) -> ContactList<Group> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getGroups", &[])
            .unwrap();
        ContactList::from_instance(instance)
    }
    pub fn get_logger() -> MiraiLogger {
        todo!("get logger")
    }
    pub fn get_other_clients(&self) -> ContactList<OtherClient> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getOtherClients", &[])
            .unwrap();
        ContactList::from_instance(instance)
    }
    pub fn get_stranger(&self, id: i64) -> Option<Stranger> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "getStranger",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Some(Stranger::from_instance(instance))
        } else {
            None
        }
    }
    pub fn get_strangers(&self) -> ContactList<Stranger> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getStrangers", &[])
            .unwrap();
        ContactList::from_instance(instance)
    }
    pub fn is_online(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .chain(&self.instance)
            .unwrap()
            .invoke("isOnline", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn join(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "join", &[])
            .unwrap();
    }
    pub fn login(&self) {
        Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "login", &Vec::new())
            .unwrap();
    }
}

impl ContactOrBotTrait for Bot {
    fn get_id(&self) -> i64 {
        self.id
    }
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

impl UserOrBotTrait for Bot {}

impl NudgeSupportedTrait for Bot {
    type NudgeType = BotNudge;
}
