use mj_helper_macro::mj_all;

use crate::utils::contact::file::{AbsoluteFile, AbsoluteFileFolderTrait, AbsoluteFolder};

#[mj_all("contact.file.AbsoluteFileFolder")]
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
