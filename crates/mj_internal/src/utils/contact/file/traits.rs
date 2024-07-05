use j4rs::{InvocationArg, Jvm};

use mj_base::env::{AsInstanceTrait, FromInstanceTrait, GetClassTypeTrait, GetInstanceTrait};

use crate::contact::Group;
use crate::utils::contact::file::{AbsoluteFile, AbsoluteFolder};

pub trait AbsoluteFileFolderTrait: Sized + GetInstanceTrait + AsInstanceTrait {
    fn delete(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("delete", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn exists(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("exists", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_absolute_path(&self) -> String // 这里应该是远程文件，所以先不用 PathBuf
    {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("getAbsolutePath", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    // FileSupported 当前只有 Group
    fn get_contact(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(self.as_instance(), "getContact", InvocationArg::empty()).unwrap();
        Group::from_instance(instance)
    }
    fn get_extension<T: AbsoluteFileFolderTrait>(file_or_folder: T) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(file_or_folder.as_instance())
            .unwrap()
            .invoke("getExtension", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("getId", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_last_modified_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("getLastModifiedTime", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = self.get_instance();
        jvm.chain(&instance)
            .unwrap()
            .invoke("getName", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_name_without_extension<T: AbsoluteFileFolderTrait>(file_or_folder: T) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(file_or_folder.as_instance())
            .unwrap()
            .invoke("getNameWithoutExtension", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_parent(&self) -> AbsoluteFolder {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(self.as_instance(), "getParent", InvocationArg::empty()).unwrap();
        AbsoluteFolder::from_instance(instance)
    }
    fn get_upload_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("getUploadTime", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_uploader_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("getUploaderId", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn is_file(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("isFile", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn to_file(&self) -> AbsoluteFile {
        let instance = self.get_instance();
        let instance = AbsoluteFile::cast_to_this_type(instance);
        AbsoluteFile::from_instance(instance)
    }
    fn is_folder(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("isFolder", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn to_folder(&self) -> AbsoluteFolder {
        let instance = self.get_instance();
        let instance = AbsoluteFolder::cast_to_this_type(instance);
        AbsoluteFolder::from_instance(instance)
    }
    fn refresh(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("refresh", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn refreshed(&self) -> Self;

    /// 重命名，目前会失败。
    fn rename_to(&self, name: &str) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("renameTo", &[InvocationArg::try_from(name).unwrap()])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(self.as_instance())
            .unwrap()
            .invoke("toString", InvocationArg::empty())
            .unwrap()
            .to_rust()
            .unwrap()
    }
}
