use crate::utils::contact::file::{AbsoluteFile, AbsoluteFileFolderTrait, AbsoluteFolder};
use j4rs::Instance;
use mj_base::{
    env::{AsInstanceTrait, FromInstanceTrait, GetInstanceTrait},
    utils::is_instance_of,
};
use mj_macro::{java_type, mj_all};

#[mj_all("net.mamoe.mirai.contact.file.AbsoluteFileFolder")]
pub enum AbsoluteFileFolder {
    AbsoluteFile(AbsoluteFile),
    AbsoluteFolder(AbsoluteFolder),
}

impl AbsoluteFileFolderTrait for AbsoluteFileFolder {
    fn refreshed(&self) -> Self {
        match self {
            AbsoluteFileFolder::AbsoluteFile(a) => AbsoluteFileFolder::AbsoluteFile(a.refreshed()),
            AbsoluteFileFolder::AbsoluteFolder(a) => {
                AbsoluteFileFolder::AbsoluteFolder(a.refreshed())
            }
        }
    }
}
