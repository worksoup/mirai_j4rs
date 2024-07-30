use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{utils::instance_is_null, AsInstanceTrait, FromInstanceTrait};

use mj_helper_macro::{java_fn, mj_all};

use crate::{
    contact::{
        AssertMemberPermissionTrait, ContactOrBotTrait, ContactTrait, Group, MemberPermission,
        MemberTrait, NudgeSupportedTrait, SendMessageSupportedTrait, UserOrBotTrait, UserTrait,
    },
    utils::{backend::BotBackend, other::enums::AvatarSpec},
};

#[mj_all("contact.NormalMember")]
pub struct NormalMember<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> AssertMemberPermissionTrait<B> for NormalMember<B> {
    fn is_owner(&self) -> bool {
        self.get_permission().eq(&MemberPermission::Owner)
    }

    fn is_administrator(&self) -> bool {
        self.get_permission().eq(&MemberPermission::Administrator)
    }

    fn is_operator(&self) -> bool {
        self.is_administrator() || self.is_owner()
    }
}

/// 没有实现 `asFriend` 所以如果需要此功能，暂时可以在获取 id 之后在 [Bot] 上调用 `get_friends`, 然后取 [Friend].
impl<B: BotBackend> NormalMember<B> {
    #[java_fn("getOwner")]
    pub fn owner_of(group: &Group<B>) -> Self {}
    #[java_fn("getBotAsMember")]
    pub fn bot_in(group: &Group<B>) -> Self {}
    pub fn in_group(group: &Group<B>, id: i64) -> Option<Self> {
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(
                group.as_instance(),
                "get",
                &[InvocationArg::try_from(id)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if !instance_is_null(&instance) {
            Some(NormalMember::from_instance(instance))
        } else {
            None
        }
    }
    pub fn get_mute_time_remaining(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let time = jvm
            .invoke(
                &self.instance,
                "getMuteTimeRemaining",
                InvocationArg::empty(),
            )
            .unwrap();
        jvm.to_rust(time).unwrap()
    }
    pub fn is_muted(&self) -> bool {
        self.get_mute_time_remaining() != 0
    }
    pub fn get_join_timestamp(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let time = jvm
            .invoke(&self.instance, "getJoinTimestamp", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(time).unwrap()
    }
    pub fn get_last_speak_timestamp(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let time = jvm
            .invoke(
                &self.instance,
                "getLastSpeakTimestamp",
                InvocationArg::empty(),
            )
            .unwrap();
        jvm.to_rust(time).unwrap()
    }
    pub fn unmute(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm
            .invoke(&self.instance, "unmute", InvocationArg::empty())
            .unwrap();
    }
    pub fn kick(&self, message: &str, block: bool) {
        let jvm = Jvm::attach_thread().unwrap();
        let message = InvocationArg::try_from(message).unwrap();
        let block = InvocationArg::try_from(block)
            .unwrap()
            .into_primitive()
            .unwrap();
        let _ = jvm
            .invoke(&self.instance, "unmute", &[message, block])
            .unwrap();
    }
    pub fn modify_admin(&self, op: bool) {
        let jvm = Jvm::attach_thread().unwrap();
        let op = InvocationArg::try_from(op)
            .unwrap()
            .into_primitive()
            .unwrap();
        let _ = jvm.invoke(&self.instance, "modifyAdmin", &[op]).unwrap();
    }
}

impl<B: BotBackend> ContactOrBotTrait<B> for NormalMember<B> {
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

impl<B: BotBackend> ContactTrait<B> for NormalMember<B> {}

impl<B: BotBackend> SendMessageSupportedTrait<B> for NormalMember<B> {}

impl<B: BotBackend> UserOrBotTrait<B> for NormalMember<B> {}

impl<B: BotBackend> NudgeSupportedTrait<B> for NormalMember<B> {}

impl<B: BotBackend> UserTrait<B> for NormalMember<B> {}

impl<B: BotBackend> MemberTrait<B> for NormalMember<B> {}
