use crate::contact::file::{AbsoluteFile, AbsoluteFileFolderTrait, AbsoluteFolder};
use j4rs::Instance;
use mj_base::env::{AsInstanceTrait, FromInstance, GetInstanceTrait};
use mj_base::utils::is_instance_of;
use mj_macro::java_type;

#[java_type("net.mamoe.mirai.contact.file.AbsoluteFileFolder")]
pub enum AbsoluteFileFolder {
    AbsoluteFile(AbsoluteFile),
    AbsoluteFolder(AbsoluteFolder),
}

impl FromInstance for AbsoluteFileFolder {
    fn from_instance(instance: Instance) -> Self {
        if is_instance_of(&instance, "net.mamoe.mirai.contact.file.AbsoluteFile") {
            AbsoluteFileFolder::AbsoluteFile(AbsoluteFile::from_instance(instance))
        } else {
            AbsoluteFileFolder::AbsoluteFolder(AbsoluteFolder::from_instance(instance))
        }
    }
}

impl GetInstanceTrait for AbsoluteFileFolder {
    fn get_instance(&self) -> Instance {
        match self {
            AbsoluteFileFolder::AbsoluteFile(a) => a.get_instance(),
            AbsoluteFileFolder::AbsoluteFolder(a) => a.get_instance(),
        }
    }
}

impl AsInstanceTrait for AbsoluteFileFolder {
    fn as_instance(&self) -> &Instance {
        match self {
            AbsoluteFileFolder::AbsoluteFile(a) => a.as_instance(),
            AbsoluteFileFolder::AbsoluteFolder(a) => a.as_instance(),
        }
    }
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
