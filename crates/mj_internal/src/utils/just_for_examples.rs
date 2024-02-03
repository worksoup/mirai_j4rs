use crate::contact::{Env, EnvConfig};
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

pub fn get_test_bot(working_dir: &str) -> (Bot, i64, i64) {
    let config: Config = toml::from_str(
        std::fs::read_to_string(Path::new(working_dir).join("config.toml"))
            .unwrap()
            .as_str(),
    )
    .unwrap();
    let env_config: EnvConfig = toml::from_str(
        &std::fs::read_to_string(Path::new(working_dir).join("env_config.toml")).unwrap(),
    )
    .unwrap();
    let group_id = config.group_id;
    let member_id = config.member_id;
    let bot_authorization = if !config.passwd.is_empty() {
        BotAuthorization::Password(config.passwd.clone())
    } else {
        BotAuthorization::QrCode
    };
    (
        BotBuilder::new(&env_config)
            .id(config.id)
            .authorization(bot_authorization)
            .file_based_device_info(None)
            .set_protocol(MiraiProtocol::W)
            .set_working_dir(working_dir)
            .build(),
        group_id,
        member_id,
    )
}

#[cfg(test)]
mod tests {}
