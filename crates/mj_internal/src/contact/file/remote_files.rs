use crate::contact::{
    file::{AbsoluteFile, AbsoluteFolder, ExternalResource},
    group::Group,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{FromInstance, GetInstanceTrait};
use mj_macro::{FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, FromInstanceDerive)]
pub struct RemoteFiles {
    instance: Instance,
}

impl RemoteFiles {
    /// 该函数返回 FileSupported, 但是目前应该只有 Group 的吧？
    pub fn get_contact(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getContact", &[]).unwrap();
        let instance = jvm
            .cast(&instance, "net.mamoe.mirai.contact.Group")
            .unwrap();
        let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
        let id = jvm
            .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
            .unwrap();
        Group { bot, instance, id }
    }
    pub fn get_root(&self) -> AbsoluteFolder {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getRoot", &[]).unwrap();
        AbsoluteFolder::from_instance(instance)
    }

    /// 上传新文件。
    pub fn upload_new_file(&self, file_name: &str, resource: &ExternalResource) -> AbsoluteFile {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(
                &self.instance,
                "uploadNewFile",
                &[
                    InvocationArg::try_from(file_name).unwrap(),
                    InvocationArg::try_from(jvm.clone_instance(&resource.get_instance()).unwrap())
                        .unwrap(),
                ],
            )
            .unwrap();
        AbsoluteFile::from_instance(instance)
    }

    /// 上传新文件，传入的 callback 可以获取到当前上传文件的进度。
    pub fn upload_new_file_with_progression_callback() -> AbsoluteFile {
        todo!()
    }
}
