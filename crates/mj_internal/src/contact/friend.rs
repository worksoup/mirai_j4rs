use j4rs::{errors::J4RsError, Instance, InvocationArg, Jvm};
use jbuchong::{java, AsInstanceTrait, TryFromInstanceTrait};

use crate::{
    contact::{
        Bot, ContactOrBotTrait, ContactTrait, NudgeSupportedTrait, SendMessageSupportedTrait,
        UserOrBotTrait, UserTrait,
    },
    utils::{contact::friend_group::FriendGroup, other::enums::AvatarSpec},
};

#[java("net.mamoe.mirai.contact.Friend")]
pub struct Friend {
    bot: Bot,
    instance: Instance,
    id: i64,
}

impl TryFromInstanceTrait for Friend {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        let bot = jvm
            .invoke(&instance, "getBot", InvocationArg::empty())
            .unwrap();
        let bot = Bot::try_from_instance(bot).unwrap();
        let id = jvm
            .to_rust(
                jvm.invoke(&instance, "getId", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap();
        Ok(Friend { bot, instance, id })
    }
}
impl SendMessageSupportedTrait for Friend {}

impl Friend {
    pub fn from_bot(bot: &Bot) -> Self {
        let id = bot.get_id();
        let instance = Jvm::attach_thread()
            .unwrap()
            .invoke(bot.as_instance(), "getAsFriend", InvocationArg::empty())
            .unwrap();
        Friend {
            bot: bot.clone(),
            instance,
            id,
        }
    }
    pub fn delete(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm
            .invoke(&self.instance, "delete", InvocationArg::empty())
            .unwrap();
    }
    pub fn get_friend_group(&self) -> FriendGroup {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getFriendGroup", InvocationArg::empty())
            .unwrap();
        FriendGroup { instance }
    }
    pub fn get_remark(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getRemark", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn set_remark(&self, remark: &str) {
        let jvm = Jvm::attach_thread().unwrap();
        let remark = InvocationArg::try_from(remark).unwrap();
        let _ = jvm.invoke(&self.instance, "delete", &[remark]).unwrap();
    }
}

impl ContactOrBotTrait for Friend {
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

impl UserOrBotTrait for Friend {}

impl NudgeSupportedTrait for Friend {}

impl ContactTrait for Friend {}

impl UserTrait for Friend {}
