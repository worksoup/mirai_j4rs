use crate::utils::backend::BotBackend;
use crate::utils::data_wrapper::DataWrapper;
use crate::{
    contact::{
        Bot, ContactOrBotTrait, ContactTrait, NudgeSupportedTrait, SendMessageSupportedTrait,
        UserOrBotTrait, UserTrait,
    },
    utils::{contact::friend_group::FriendGroup, other::enums::AvatarSpec},
};
use j4rs::{Instance, InvocationArg};
use mj_helper_macro::{java_fn, mj_all};

#[mj_all("contact.Friend")]
pub struct Friend<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> SendMessageSupportedTrait<B> for Friend<B> {}

impl<B: BotBackend> Friend<B> {
    pub fn from_bot(bot: &Bot<B>) -> Self {
        bot.get_as_friend()
    }
    #[java_fn]
    pub fn delete(&self) {}
    #[java_fn]
    pub fn get_friend_group(&self) -> FriendGroup<B> {}
    #[java_fn]
    pub fn get_remark(&self) -> String {}
    #[java_fn]
    pub fn set_remark(&self, remark: DataWrapper<&str>) {}
}

impl<B: BotBackend> ContactOrBotTrait<B> for Friend<B> {
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

impl<B: BotBackend> UserOrBotTrait<B> for Friend<B> {}

impl<B: BotBackend> NudgeSupportedTrait<B> for Friend<B> {}

impl<B: BotBackend> ContactTrait<B> for Friend<B> {}

impl<B: BotBackend> UserTrait<B> for Friend<B> {}
