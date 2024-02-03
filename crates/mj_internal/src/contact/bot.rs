use crate::utils::{BotConfiguration, MiraiLogger};
use crate::{
    contact::{
        contact_trait::{ContactOrBotTrait, NudgeSupportedTrait, UserOrBotTrait},
        group::Group,
        ContactList, Friend, OtherClient, Stranger,
    },
    error::MiraiRsError,
    event::EventChannel,
    message::action::BotNudge,
    utils::{contact::friend_group::FriendGroups, other::enums::AvatarSpec},
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::{
    env::{FromInstance, GetInstanceTrait},
    utils::instance_is_null,
};

pub struct Bot {
    pub(crate) instance: Instance,
    pub(crate) id: i64,
}

impl FromInstance for Bot {
    fn from_instance(instance: Instance) -> Self {
        let id = Jvm::attach_thread()
            .unwrap()
            .chain(&instance)
            .unwrap()
            .invoke("getId", &[])
            .unwrap()
            .to_rust()
            .unwrap();
        Bot { instance, id }
    }
}

impl GetInstanceTrait for Bot {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl Bot {
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
        let id = self.id;
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(&self.instance, "getAsFriend", &[])
            .unwrap();
        Friend {
            bot: self.get_instance(),
            instance,
            id,
        }
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
            Some(Friend {
                bot: self.get_instance(),
                instance,
                id,
            })
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
