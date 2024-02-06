use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{AsInstanceTrait, FromInstanceTrait};
use mj_base::utils::instance_is_null;
use mj_macro::{java_type, AsInstanceDerive, GetInstanceDerive};

use crate::contact::{
    AssertMemberPermissionTrait, Bot, ContactOrBotTrait, ContactTrait, Friend, Group,
    MemberPermission, MemberTrait, NudgeSupportedTrait, SendMessageSupportedTrait, UserOrBotTrait,
    UserTrait,
};
use crate::utils::other::enums::AvatarSpec;

#[derive(GetInstanceDerive, AsInstanceDerive)]
#[java_type("contact.NormalMember")]
pub struct NormalMember {
    bot: Bot,
    instance: Instance,
    id: i64,
}

impl AssertMemberPermissionTrait for NormalMember {
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

impl FromInstanceTrait for NormalMember {
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let bot = Bot::from_instance(bot);
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        NormalMember { bot, instance, id }
    }
}

/// 没有实现 `asFriend` 所以如果需要此功能，暂时可以在获取 id 之后在 [Bot] 上调用 `get_friends`, 然后取 [Friend].
impl NormalMember {
    pub fn owner_of(group: &Group) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(group.as_instance(), "getOwner", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        NormalMember {
            bot: group.get_bot(),
            instance,
            id,
        }
    }
    pub fn bot_in(group: &Group) -> Self {
        let bot = group.get_bot();
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(group.as_instance(), "getBotAsMember", &[])
            .unwrap();
        let id = Jvm::attach_thread()
            .unwrap()
            .to_rust(
                Jvm::attach_thread()
                    .unwrap()
                    .invoke(bot.as_instance(), "getId", &[])
                    .unwrap(),
            )
            .unwrap();
        NormalMember { bot, instance, id }
    }
    pub fn in_group(group: &Group, id: i64) -> Option<Self> {
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
            Some(NormalMember {
                bot: group.get_bot(),
                instance,
                id,
            })
        } else {
            None
        }
    }
    pub fn get_mute_time_remaining(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let time = jvm
            .invoke(&self.instance, "getMuteTimeRemaining", &[])
            .unwrap();
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
        let time = jvm
            .invoke(&self.instance, "getLastSpeakTimestamp", &[])
            .unwrap();
        jvm.to_rust(time).unwrap()
    }
    pub fn unmute(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "unmute", &[]).unwrap();
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

impl ContactOrBotTrait for NormalMember {
    fn get_bot(&self) -> Bot {
        self.bot.clone()
    }
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

impl SendMessageSupportedTrait for NormalMember {}

impl UserOrBotTrait for NormalMember {}

impl NudgeSupportedTrait for NormalMember {}

impl UserTrait for NormalMember {}

impl MemberTrait for NormalMember {}
