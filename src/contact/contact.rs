use std::marker::PhantomData;
//联系人
use super::bot::{Bot, Env};
use super::contact_trait::{
    ContactOrBotTrait, ContactTrait, MemberTrait, UserOrBotTrait, UserTrait,
};
use crate::action::nudges::FriendNudge;
use crate::contact::bot::FriendGroup;
use crate::env::{FromInstance, GetBotTrait, GetEnvTrait};
use crate::message::message_trait::MessageHashCodeTrait;
use crate::utils::internal::instance_is_null;
use crate::{env::ContactFromInstance, other::enums::AvatarSpec};
use contact_derive::{GetBotDerive, GetInstanceDerive};
use j4rs::{Instance, InvocationArg, Jvm};

pub struct ContactList<T>
    where
        T: ContactTrait + ContactFromInstance,
{
    pub(crate) bot: Instance,
    pub(crate) instance: Instance,
    pub(crate) _unused: PhantomData<T>,
}

impl<T: ContactTrait + ContactFromInstance> GetEnvTrait for ContactList<T> {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<T> ContactList<T>
    where
        T: ContactTrait + ContactFromInstance,
{
    pub fn contains(&self, contact: T) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.instance,
                        "contains",
                        &[InvocationArg::try_from(contact.get_instance()).unwrap()],
                    )
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn contains_id(&self, id: i64) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(
                        &self.instance,
                        "contains",
                        &[InvocationArg::try_from(id)
                            .unwrap()
                            .into_primitive()
                            .unwrap()],
                    )
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn get(self, id: i64) -> Option<T::Item> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "get",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if instance_is_null(&instance) {
            None
        } else {
            Some(T::from_instance(self.bot, instance, id))
        }
    }
    pub fn get_size(&self) -> usize {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "getSize", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn is_empty(&self) -> bool {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "isEmpty", &[])
                    .unwrap(),
            )
            .unwrap()
    }
    pub fn to_string(&self) -> String {
        Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(&self.instance, "toString", &[])
                    .unwrap(),
            )
            .unwrap()
    }
}

impl<T: ContactTrait + ContactFromInstance> MessageHashCodeTrait for ContactList<T> {}
// TODO: impl MiraiRsCollectionTrait fot ContactList<_>{}

#[derive(GetBotDerive, GetInstanceDerive)]
pub struct Friend {
    pub(crate) bot: Instance,
    pub(crate) instance: Instance,
    pub(crate) id: i64,
}

impl Friend {
    pub fn delete(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "delete", &[]).unwrap();
    }
    pub fn get_friend_group(&self) -> FriendGroup {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getFriendGroup", &[]).unwrap();
        FriendGroup { instance }
    }
    pub fn get_remark(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getRemark", &[]).unwrap())
            .unwrap()
    }
    pub fn nudge(&self) -> FriendNudge {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "nudge", &[]).unwrap();
        FriendNudge { instance }
    }
    pub fn set_remark(&self, remark: &str) {
        let jvm = Jvm::attach_thread().unwrap();
        let remark = InvocationArg::try_from(remark).unwrap();
        let _ = jvm.invoke(&self.instance, "delete", &[remark]).unwrap();
    }
}

impl FromInstance for Friend {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        Friend { bot, instance, id }
    }
}

impl ContactFromInstance for Friend {
    type Item = Friend;
    fn from_instance(bot: Instance, instance: Instance, id: i64) -> Friend {
        Friend { bot, instance, id }
    }
}

impl ContactOrBotTrait for Friend {
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
        "https://q.qlogo.cn/g?b=qq&nk=".to_string()
            + self.get_id().to_string().as_str()
            + "&s="
            + size.to_string().as_str()
    }
}

impl UserOrBotTrait for Friend {}

impl ContactTrait for Friend {}

impl UserTrait for Friend {}

#[derive(GetBotDerive, GetInstanceDerive)]
pub struct Stranger {
    pub(crate) bot: Instance,
    pub(crate) instance: Instance,
    pub(crate) id: i64,
}

impl ContactFromInstance for Stranger {
    type Item = Stranger;
    fn from_instance(bot: Instance, instance: Instance, id: i64) -> Stranger {
        Stranger { bot, instance, id }
    }
}

impl ContactOrBotTrait for Stranger {
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
        "https://q.qlogo.cn/g?b=qq&nk=".to_string()
            + self.get_id().to_string().as_str()
            + "&s="
            + size.to_string().as_str()
    }
}

impl UserOrBotTrait for Stranger {}

impl ContactTrait for Stranger {}

impl UserTrait for Stranger {}

#[derive(GetBotDerive, GetInstanceDerive)]
pub struct OtherClient {
    bot: Instance,
    instance: Instance,
}

impl ContactFromInstance for OtherClient {
    type Item = OtherClient;
    fn from_instance(bot: Instance, instance: Instance, _id: i64) -> OtherClient {
        OtherClient { bot, instance }
    }
}

impl ContactOrBotTrait for OtherClient {}

impl ContactTrait for OtherClient {}

#[derive(GetInstanceDerive, GetBotDerive)]
pub struct NormalMember {
    pub(crate) bot: Instance,
    pub(crate) instance: Instance,
    pub(crate) id: i64,
}

impl ContactFromInstance for NormalMember {
    type Item = NormalMember;
    fn from_instance(bot: Instance, instance: Instance, id: i64) -> NormalMember {
        NormalMember { bot, instance, id }
    }
}

impl ContactOrBotTrait for NormalMember {
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
        "https://q.qlogo.cn/g?b=qq&nk=".to_string()
            + self.get_id().to_string().as_str()
            + "&s="
            + size.to_string().as_str()
    }
}

impl ContactTrait for NormalMember {}

impl UserOrBotTrait for NormalMember {}

impl UserTrait for NormalMember {}

impl MemberTrait for NormalMember {}

#[derive(GetBotDerive, GetInstanceDerive)]
pub struct AnonymousMember {
    pub(crate) bot: Instance,
    pub(crate) instance: Instance,
    pub(crate) id: i64,
}

impl ContactOrBotTrait for AnonymousMember {}

impl ContactTrait for AnonymousMember {}

impl UserOrBotTrait for AnonymousMember {}

impl UserTrait for AnonymousMember {}

pub enum Member {
    NormalMember(NormalMember),
    AnonymousMember(AnonymousMember),
}

impl GetBotTrait for Member {
    fn get_bot(&self) -> Bot {
        match self {
            Member::NormalMember(member) => member.get_bot(),
            Member::AnonymousMember(member) => member.get_bot(),
        }
    }
}

impl GetEnvTrait for Member {
    fn get_instance(&self) -> Instance {
        match self {
            Member::NormalMember(member) => member.get_instance(),
            Member::AnonymousMember(member) => member.get_instance(),
        }
    }
}

impl ContactOrBotTrait for Member {}

impl ContactTrait for Member {}

impl UserOrBotTrait for Member {}

impl UserTrait for Member {}
