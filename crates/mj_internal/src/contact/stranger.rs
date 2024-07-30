use j4rs::Instance;

use mj_helper_macro::mj_all;

use crate::contact::{
    ContactOrBotTrait, ContactTrait, NudgeSupportedTrait, SendMessageSupportedTrait,
    UserOrBotTrait, UserTrait,
};
use crate::utils::backend::BotBackend;
use crate::utils::other::enums::AvatarSpec;

#[mj_all("contact.Stranger")]
pub struct Stranger<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> ContactOrBotTrait<B> for Stranger<B> {
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

impl<B: BotBackend> UserOrBotTrait<B> for Stranger<B> {}

impl<B: BotBackend> NudgeSupportedTrait<B> for Stranger<B> {}

impl<B: BotBackend> ContactTrait<B> for Stranger<B> {}

impl<B: BotBackend> SendMessageSupportedTrait<B> for Stranger<B> {}

impl<B: BotBackend> UserTrait<B> for Stranger<B> {}
