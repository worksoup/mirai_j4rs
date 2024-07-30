use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::utils::backend::{Mirai, Overflow};
use crate::utils::bot_builder::BotBuilder;
use crate::{
    auth::bot_authorization::BotAuthorization, contact::Bot, utils::other::enums::MiraiProtocol,
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub id: i64,
    pub passwd: String,
    pub group_id: i64,
    pub member_id: i64,
}
pub fn bot_group_member(working_dir: &str) -> (Bot<Mirai>, i64, i64) {
    let config: Config = toml::from_str(
        std::fs::read_to_string(Path::new(working_dir).join("config.toml"))
            .unwrap()
            .as_str(),
    )
    .unwrap();
    let group_id = config.group_id;
    let member_id = config.member_id;
    let bot_builder = BotBuilder::<Mirai>::new(working_dir);
    let bot_authorization = if !config.passwd.is_empty() {
        BotAuthorization::Password(config.passwd.clone())
    } else {
        BotAuthorization::QrCode
    };
    (
        bot_builder
            .id(config.id)
            .authorization(bot_authorization)
            .file_based_device_info(None)
            .protocol(MiraiProtocol::W)
            .working_dir(working_dir)
            .build(),
        group_id,
        member_id,
    )
}
pub fn bot_group_member_overflow(working_dir: &str, port: i32) -> (Bot<Overflow>, i64, i64) {
    let config: Config = toml::from_str(
        std::fs::read_to_string(Path::new(working_dir).join("config.toml"))
            .unwrap()
            .as_str(),
    )
    .unwrap();
    let group_id = config.group_id;
    let member_id = config.member_id;
    let bot_builder = BotBuilder::<Overflow>::reversed(working_dir, port);
    (bot_builder.connect(), group_id, member_id)
}
#[cfg(test)]
mod tests {}
