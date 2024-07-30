use crate::utils::backend::BotBackend;
use crate::utils::contact::file::{AbsoluteFile, AbsoluteFileFolderTrait, AbsoluteFolder};
use mj_helper_macro::mj_all;

#[mj_all("contact.file.AbsoluteFileFolder")]
pub enum AbsoluteFileFolder<B: BotBackend> {
    AbsoluteFile(AbsoluteFile<B>),
    AbsoluteFolder(AbsoluteFolder<B>),
}

impl<B: BotBackend> AbsoluteFileFolderTrait<B> for AbsoluteFileFolder<B> {
    fn refreshed(&self) -> Self {
        match self {
            AbsoluteFileFolder::AbsoluteFile(a) => AbsoluteFileFolder::AbsoluteFile(a.refreshed()),
            AbsoluteFileFolder::AbsoluteFolder(a) => {
                AbsoluteFileFolder::AbsoluteFolder(a.refreshed())
            }
        }
    }
}
