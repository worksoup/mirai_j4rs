use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{AsInstanceTrait, FromInstanceTrait};
use mj_helper_macro::mj_all;

use crate::utils::backend::{BotBackend, Mirai, Overflow};
use crate::utils::{
    contact::file::{AbsoluteFile, AbsoluteFileFolder, AbsoluteFileFolderTrait, ExternalResource},
    JavaStream,
};

/// # 绝对目录标识。
/// 精确表示一个远程目录。不会受同名文件或目录的影响。
/// Mirai 中有些方法会返回 Flow 或 Stream, 后者的方法名称有 Stream 后缀，
/// 这里包装的全部都是 Stream 版本，哪怕没有后缀。这些方法会返回一个迭代器，以此模拟其操作。
#[mj_all("contact.file.AbsoluteFolder")]
pub struct AbsoluteFolder<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl AbsoluteFolder<Overflow> {
    pub fn children(&self) -> JavaStream<AbsoluteFileFolder<Overflow>> {
        // waiting for overflow's impl.
        todo!()
    }
    /// 上传新文件。
    pub fn upload_new_file(
        &self,
        _file_name: &str,
        _resource: &ExternalResource,
    ) -> AbsoluteFile<Overflow> {
        // waiting for overflow's impl.
        todo!()
    }
}
impl AbsoluteFolder<Mirai> {
    pub fn children(&self) -> JavaStream<AbsoluteFileFolder<Mirai>> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "childrenStream", InvocationArg::empty())
            .unwrap();
        JavaStream {
            instance,
            _unused: Default::default(),
        }
    }
    /// 上传新文件。
    pub fn upload_new_file(
        &self,
        file_name: &str,
        resource: &ExternalResource,
    ) -> AbsoluteFile<Mirai> {
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
}
impl<B: BotBackend> AbsoluteFolder<B> {
    pub fn create_folder(&self, folder_name: &str) -> AbsoluteFolder<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let folder_name = InvocationArg::try_from(folder_name).unwrap();
        let instance = jvm
            .invoke(&self.instance, "createFolder", &[folder_name])
            .unwrap();
        AbsoluteFolder::from_instance(instance)
    }
    pub fn files(&self) -> JavaStream<AbsoluteFile<B>> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "filesStream", InvocationArg::empty())
            .unwrap();
        JavaStream {
            instance,
            _unused: Default::default(),
        }
    }
    pub fn folders(&self) -> JavaStream<AbsoluteFolder<B>> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "foldersStream", InvocationArg::empty())
            .unwrap();
        JavaStream {
            instance,
            _unused: Default::default(),
        }
    }
    pub fn get_contents_count(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getContentsCount", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn is_empty(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "isEmpty", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn resolve_all(&self, path: &str) -> JavaStream<AbsoluteFileFolder<B>> {
        let jvm = Jvm::attach_thread().unwrap();
        let path = InvocationArg::try_from(path).unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveAllStream", &[path])
            .unwrap();
        JavaStream {
            instance,
            _unused: Default::default(),
        }
    }
    pub fn resolve_file_by_id(&self, id: &str, deep: bool) -> AbsoluteFile<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let id = InvocationArg::try_from(id).unwrap();
        let deep = InvocationArg::try_from(deep)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveFileById", &[id, deep])
            .unwrap();
        AbsoluteFile::from_instance(instance)
    }
    pub fn resolve_files(&self, path: &str) -> JavaStream<AbsoluteFile<B>> {
        let jvm = Jvm::attach_thread().unwrap();
        let path = InvocationArg::try_from(path).unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveFilesStream", &[path])
            .unwrap();
        JavaStream {
            instance,
            _unused: Default::default(),
        }
    }
    pub fn resolve_folder(&self, path: &str) -> AbsoluteFolder<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let path = InvocationArg::try_from(path).unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveFolder", &[path])
            .unwrap();
        AbsoluteFolder::from_instance(instance)
    }
    pub fn resolve_folder_by_id(&self, id: &str) -> AbsoluteFolder<B> {
        let jvm = Jvm::attach_thread().unwrap();
        let id = InvocationArg::try_from(id).unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveFolderById", &[id])
            .unwrap();
        AbsoluteFolder::from_instance(instance)
    }
    /// 上传新文件，传入的 callback 可以获取到当前上传文件的进度。
    pub fn upload_new_file_with_progression_callback() -> AbsoluteFile<B> {
        todo!()
    }
}

impl<B: BotBackend> AbsoluteFileFolderTrait<B> for AbsoluteFolder<B> {
    fn refreshed(&self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "refreshed", InvocationArg::empty())
            .unwrap();
        AbsoluteFolder::from_instance(instance)
    }
}
