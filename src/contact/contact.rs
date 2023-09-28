use std::marker::PhantomData;
use std::path::PathBuf;
//联系人
use super::contact_trait::{
    ContactOrBotTrait, ContactTrait, MemberTrait, UserOrBotTrait, UserTrait,
};
use crate::action::nudges::{FriendNudge, MemberNudge};
use crate::contact::bot::FriendGroup;
use crate::env::{FromInstance, GetEnvTrait};
use crate::message::message_trait::{MessageHashCodeTrait, MessageTrait};
use crate::utils::{internal::instance_is_null, other::enums::AvatarSpec};
use contact_derive::{GetInstanceDerive};
use j4rs::{Instance, InvocationArg, Jvm};
use crate::message::{Image, MessageReceipt};

pub struct ContactList<T>
    where
        T: ContactTrait + FromInstance,
{
    pub(crate) instance: Instance,
    pub(crate) _unused: PhantomData<T>,
}

impl<T: ContactTrait + FromInstance> FromInstance for ContactList<T> {
    fn from_instance(instance: Instance) -> Self {
        ContactList {
            instance,
            _unused: PhantomData::default(),
        }
    }
}

impl<T: ContactTrait + FromInstance> GetEnvTrait for ContactList<T> {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<T> ContactList<T>
    where
        T: ContactTrait + FromInstance,
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
    pub fn get(self, id: i64) -> Option<T> {
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
            Some(T::from_instance(instance))
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

impl<T: ContactTrait + FromInstance> MessageHashCodeTrait for ContactList<T> {}
// TODO: impl MiraiRsCollectionTrait fot ContactList<_>{}

#[derive(GetInstanceDerive)]
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

impl UserOrBotTrait for Friend {
    type NudgeType = FriendNudge;
    fn nudge(&self) -> FriendNudge {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "nudge", &[]).unwrap();
        FriendNudge { instance }
    }
}

impl ContactTrait for Friend {}

impl UserTrait for Friend {}

#[derive(GetInstanceDerive)]
pub struct Stranger {
    pub(crate) instance: Instance,
}

// impl GetBotTrait for Stranger {
//     fn get_bot(&self) -> Bot {
//         let instance = Jvm::attach_thread().unwrap().invoke(&self.get_instance(), "getBot", &[]).unwrap();
//         Bot::from_instance(instance)
//     }
// }

impl FromInstance for Stranger {
    fn from_instance(instance: Instance) -> Self {
        Stranger { instance }
    }
}

impl ContactOrBotTrait for Stranger {
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

impl UserOrBotTrait for Stranger {
    // `StrangerNudge`
    type NudgeType = ();

    fn nudge(&self) -> Self::NudgeType {
        todo!()
    }
}

impl ContactTrait for Stranger {}

impl UserTrait for Stranger {}

#[derive(GetInstanceDerive)]
pub struct OtherClient {
    instance: Instance,
}

// impl GetBotTrait for OtherClient {
//     fn get_bot(&self) -> Bot {
//         let instance = Jvm::attach_thread().unwrap().invoke(&self.get_instance(), "getBot", &[]).unwrap();
//         Bot::from_instance(instance)
//     }
// }

impl FromInstance for OtherClient {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl ContactOrBotTrait for OtherClient {}

impl ContactTrait for OtherClient {}

#[derive(GetInstanceDerive)]
pub struct NormalMember {
    pub(crate) bot: Instance,
    pub(crate) instance: Instance,
    pub(crate) id: i64,
}

impl FromInstance for NormalMember {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm.to_rust(jvm.invoke(&instance, "getId", &[]).unwrap()).unwrap();
        NormalMember { bot, instance, id }
    }
}

/// 没有实现 `asFriend` 所以如果需要此功能，暂时可以在获取 id 之后在 [Bot] 上调用 `get_friends`, 然后取 [Friend].
impl NormalMember {
    pub fn get_mute_time_remaining(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let time = jvm.invoke(&self.instance, "getMuteTimeRemaining", &[]).unwrap();
        jvm.to_rust(time).unwrap()
    }
    pub fn is_muted(&self) -> bool {
        self.get_mute_time_remaining() != 0
    }
    pub fn get_join_timestamp(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let time = jvm.invoke(&self.instance, "getJoinTimestamp", &[]).unwrap();
        jvm.to_rust(time).unwrap()
    }
    pub fn get_last_speak_timestamp(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let time = jvm.invoke(&self.instance, "getLastSpeakTimestamp", &[]).unwrap();
        jvm.to_rust(time).unwrap()
    }
    pub fn unmute(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "unmute", &[]).unwrap();
    }
    pub fn kick(&self, message: &str, block: bool) {
        let jvm = Jvm::attach_thread().unwrap();
        let message = InvocationArg::try_from(message).unwrap();
        let block = InvocationArg::try_from(block).unwrap().into_primitive().unwrap();
        let _ = jvm.invoke(&self.instance, "unmute", &[message, block]).unwrap();
    }
    pub fn modify_admin(&self, op: bool) {
        let jvm = Jvm::attach_thread().unwrap();
        let op = InvocationArg::try_from(op).unwrap().into_primitive().unwrap();
        let _ = jvm.invoke(&self.instance, "modifyAdmin", &[op]).unwrap();
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

impl UserOrBotTrait for NormalMember {
    type NudgeType = MemberNudge;

    fn nudge(&self) -> Self::NudgeType {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "nudge", &[]).unwrap();
        Self::NudgeType { instance }
    }
}

impl UserTrait for NormalMember {}

impl MemberTrait for NormalMember {}

#[derive(GetInstanceDerive)]
pub struct AnonymousMember {
    pub(crate) instance: Instance,
}

// impl GetBotTrait for AnonymousMember {}

impl AnonymousMember {
    pub fn get_anonymous_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let id = jvm.invoke(&self.instance, "getAnonymousId", &[]).unwrap();
        jvm.to_rust(id).unwrap()
    }
}

impl FromInstance for AnonymousMember {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

impl MemberTrait for AnonymousMember {}

impl ContactOrBotTrait for AnonymousMember {}

impl ContactTrait for AnonymousMember {
    fn send_message<'a>(&self, message: impl MessageTrait) -> MessageReceipt<Self> {
        todo!("该函数不应当使用。日后重构或为此函数添加警告。")
    }
    fn send_string(&self, string: &str) -> MessageReceipt<'_, Self> {
        todo!("该函数不应当使用。日后重构或为此函数添加警告。")
    }
    fn upload_image_from_file(&self, path: &PathBuf) -> Image {
        todo!("该函数不应当使用。日后重构或为此函数添加警告。")
    }
}

impl UserOrBotTrait for AnonymousMember {
    type NudgeType = MemberNudge;

    fn nudge(&self) -> Self::NudgeType {
        todo!("该函数不应当使用。日后重构或为此函数添加警告。")
    }
}

impl UserTrait for AnonymousMember {}

pub enum Member {
    NormalMember(NormalMember),
    AnonymousMember(AnonymousMember),
}

// impl GetBotTrait for Member {
//     fn get_bot(&self) -> Bot {
//         match self {
//             Member::NormalMember(member) => member.get_bot(),
//             Member::AnonymousMember(member) => member.get_bot(),
//         }
//     }
// }

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

impl UserOrBotTrait for Member {
    type NudgeType = MemberNudge;

    fn nudge(&self) -> Self::NudgeType {
        match self {
            Member::NormalMember(a) => { a.nudge() }
            Member::AnonymousMember(a) => { a.nudge() }
        }
    }
}

impl UserTrait for Member {}
