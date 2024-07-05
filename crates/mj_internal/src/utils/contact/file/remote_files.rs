use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::{AsInstanceTrait, FromInstanceTrait, GetClassTypeTrait};
use mj_macro::{AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

use crate::contact::Group;
use crate::utils::contact::file::{AbsoluteFile, AbsoluteFolder, ExternalResource};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
pub struct RemoteFiles {
    instance: Instance,
}

impl RemoteFiles {
    /// 该函数返回 FileSupported, 但是目前应该只有 Group 的吧？
    pub fn get_contact(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getContact", InvocationArg::empty()).unwrap();
        let instance = jvm
            .cast(&instance, <Group as GetClassTypeTrait>::get_type_name().as_str())
            .unwrap();
        Group::from_instance(instance)
    }
    pub fn get_root(&self) -> AbsoluteFolder {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getRoot", InvocationArg::empty()).unwrap();
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
                    InvocationArg::try_from(jvm.clone_instance(resource.as_instance()).unwrap())
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
