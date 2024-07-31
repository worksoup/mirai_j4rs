use j4rs::{errors::J4RsError, Instance};
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use jbuchong::{GetInstanceTrait, TryFromInstanceTrait};

use crate::contact::MemberPermission;
use crate::error::MiraiRsErrorEnum::LumiaException;

#[derive(Debug, Copy, Clone)]
pub struct MemberPermissionCheck {
    required: MemberPermission,
    actual: MemberPermission,
    bot: i64,
    group: i64,
}

impl MemberPermissionCheck {
    fn permission_denied_message(&self) -> String {
        format!(
            "权限不足，需要机器人（id = {}）在群（ id = {}）内的权限至少为 {}, 当前为 {}.",
            self.bot,
            self.group,
            format_args!("{:?}", self.required),
            format_args!("{:?}", self.actual),
        )
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

#[derive(Debug, Clone)]
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
    pub fn from_j4rs_err(_j4rs_err: J4RsError) -> Self {
        todo!()
    }
    pub fn get_type(&self) -> MiraiRsErrorEnum {
        self.r#type.clone()
    }
}

impl From<J4RsError> for MiraiRsError {
    fn from(err: J4RsError) -> Self {
        Self::from_j4rs_err(err)
    }
}

impl TryFromInstanceTrait for MiraiRsError {
    fn try_from_instance(_instance: Instance) -> Result<Self, J4RsError> {
        todo!()
    }
}

impl GetInstanceTrait for MiraiRsError {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        todo!()
    }
}

impl Display for MiraiRsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Error for MiraiRsError {}
