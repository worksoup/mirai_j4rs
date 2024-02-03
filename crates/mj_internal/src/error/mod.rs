use crate::contact::MemberPermission;
use crate::error::MiraiRsErrorEnum::LumiaException;
use j4rs::errors::J4RsError;
use j4rs::Instance;
use mj_base::env::{FromInstance, GetInstanceTrait};
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct MemberPermissionCheck {
    required: MemberPermission,
    actual: MemberPermission,
    bot: i64,
    group: i64,
}

impl MemberPermissionCheck {
    fn permission_denied_message(&self) -> String {
        "权限不足，需要机器人（id = ".to_string()
            + self.bot.to_string().as_str()
            + "）在群（ id = "
            + self.group.to_string().as_str()
            + "）内的权限至少为 "
            + format!("{:?}", self.required).as_str()
            + ", 当前为"
            + format!("{:?}", self.actual).as_str()
            + "."
    }
}

impl Display for MemberPermissionCheck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = if self.actual < self.required {
            self.permission_denied_message()
        } else {
            format!("{:?}", self)
        };
        f.write_str(str.as_str())
    }
}

#[derive(Debug)]
pub enum MiraiRsErrorEnum {
    /// 权限不足。
    PermissionDenied(MemberPermissionCheck),
    /// 一般是协议异常。
    IllegalState(String),
    /// `ɒiM_J` 中出现的错误。
    LumiaException(String),
}

#[derive(Debug)]
pub struct MiraiRsError {
    pub(crate) r#type: MiraiRsErrorEnum,
    pub(crate) what: String,
}

impl MiraiRsError {
    pub fn to_lumia_exception(self) -> Self {
        Self {
            r#type: LumiaException(self.to_string()),
            what: self.what,
        }
    }
    pub fn from_j4rs_err(j4rs_err: J4RsError) -> Self {
        todo!()
    }
}

impl From<J4RsError> for MiraiRsError {
    fn from(err: J4RsError) -> Self {
        Self::from_j4rs_err(err)
    }
}

impl FromInstance for MiraiRsError {
    fn from_instance(instance: Instance) -> Self {
        todo!()
    }
}

impl GetInstanceTrait for MiraiRsError {
    fn get_instance(&self) -> Instance {
        todo!()
    }
}

impl Display for MiraiRsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Error for MiraiRsError {}
