use crate::contact::{
    ContactOrBotTrait, ContactTrait, NudgeSupportedTrait, SendMessageSupportedTrait,
    UserOrBotTrait, UserTrait,
};
use crate::utils::other::enums::AvatarSpec;
use j4rs::Instance;
use mj_macro::{mj_all, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[mj_all("net.mamoe.mirai.contact.Stranger")]
pub struct Stranger {
    instance: Instance,
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

impl UserOrBotTrait for Stranger {}

impl NudgeSupportedTrait for Stranger {}

impl ContactTrait for Stranger {}

impl SendMessageSupportedTrait for Stranger {}

impl UserTrait for Stranger {}
