use lazy_static::lazy_static;
use mirai_j4rs::auth::bot_authorization::BotAuthorization;
use mirai_j4rs::contact::bot::{Bot, BotBuilder};
use mirai_j4rs::utils::other::enums::MiraiProtocol;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub id: i64,
    pub passwd: String,
    pub group_id: i64,
    pub member_id: i64,
}

lazy_static! {
    static ref CONFIG: Config =
        toml::from_str(std::fs::read_to_string("./config.toml").unwrap().as_str()).unwrap();
}
pub fn get_test_bot() -> Bot {
    let config_dir = Path::new(".");
    let bot_authorization = if !CONFIG.passwd.is_empty() {
        BotAuthorization::Password(CONFIG.passwd.clone())
    } else {
        BotAuthorization::QrCode
    };
    BotBuilder::new(config_dir)
        .id(CONFIG.id)
        .authorization(bot_authorization)
        .file_based_device_info(None)
        .set_protocol(MiraiProtocol::W)
        .build()
}

pub fn get_group_id() -> i64 {
    CONFIG.group_id
}

pub fn get_member_id() -> i64 {
    CONFIG.member_id
}

#[cfg(test)]
mod tests {}
