use crate::{
    auth::bot_authorization::BotAuthorization,
    contact::{Bot, BotBuilder},
    utils::other::enums::MiraiProtocol,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub id: i64,
    pub passwd: String,
    pub group_id: i64,
    pub member_id: i64,
}

lazy_static! {
    static ref CONFIG: Config = toml::from_str(
        std::fs::read_to_string("./working_dir/config.toml")
            .unwrap()
            .as_str()
    )
    .unwrap();
}
pub fn get_test_bot() -> Bot {
    let config_dir = Path::new("./working_dir");
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
        .set_working_dir(&PathBuf::from("./working_dir"))
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
