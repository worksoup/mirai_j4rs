use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{
    java_all, AsInstanceTrait, FromInstanceTrait, GetClassTypeTrait, TryFromInstanceTrait,
};

use crate::utils::backend::BotBackend;
use crate::{
    contact::Group,
    utils::contact::file::{AbsoluteFile, AbsoluteFolder, ExternalResource},
};

#[java_all]
pub struct RemoteFiles<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> RemoteFiles<B> {
    /// 该函数返回 FileSupported, 但是目前应该只有 Group 的吧？
    pub fn get_contact(&self) -> Group<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getContact", InvocationArg::empty())
            .unwrap();
        let instance = jvm
            .cast(&instance, <Group<B> as GetClassTypeTrait>::get_type_name())
            .unwrap();
        Group::try_from_instance(instance).unwrap()
    }
    pub fn get_root(&self) -> AbsoluteFolder<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getRoot", InvocationArg::empty())
            .unwrap();
        AbsoluteFolder::from_instance(instance)
    }

    /// 上传新文件。
    pub fn upload_new_file(&self, file_name: &str, resource: &ExternalResource) -> AbsoluteFile<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(
                &self.instance,
                "uploadNewFile",
                &[
                    InvocationArg::try_from(file_name).unwrap(),
                    InvocationArg::from(jvm.clone_instance(resource.as_instance()).unwrap()),
                ],
            )
            .unwrap();
        AbsoluteFile::from_instance(instance)
    }

    /// 上传新文件，传入的 callback 可以获取到当前上传文件的进度。
    pub fn upload_new_file_with_progression_callback() -> AbsoluteFile<B> {
        todo!()
    }
}
