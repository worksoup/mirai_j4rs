use crate::utils::{
    contact::file::{AbsoluteFile, AbsoluteFileFolder, AbsoluteFileFolderTrait, ExternalResource},
    JavaStream,
};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::env::{AsInstanceTrait, FromInstance, GetInstanceTrait};
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

/// # 绝对目录标识。
/// 精确表示一个远程目录。不会受同名文件或目录的影响。
/// Mirai 中有些方法会返回 Flow 或 Stream, 后者的方法名称有 Stream 后缀，
/// 这里包装的全部都是 Stream 版本，哪怕没有后缀。这些方法会返回一个迭代器，以此模拟其操作。
#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.contact.file.AbsoluteFolder")]
pub struct AbsoluteFolder {
    instance: Instance,
}

impl AbsoluteFolder {
    pub fn children(&self) -> JavaStream<AbsoluteFileFolder> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "childrenStream", &[]).unwrap();
        JavaStream {
            instance,
            _unused: Default::default(),
        }
    }
    pub fn create_folder(&self, folder_name: &str) -> AbsoluteFolder {
        let jvm = Jvm::attach_thread().unwrap();
        let folder_name = InvocationArg::try_from(folder_name).unwrap();
        let instance = jvm
            .invoke(&self.instance, "createFolder", &[folder_name])
            .unwrap();
        AbsoluteFolder { instance }
    }
    pub fn files(&self) -> JavaStream<AbsoluteFile> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "filesStream", &[]).unwrap();
        JavaStream {
            instance,
            _unused: Default::default(),
        }
    }
    pub fn folders(&self) -> JavaStream<AbsoluteFolder> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "foldersStream", &[]).unwrap();
        JavaStream {
            instance,
            _unused: Default::default(),
        }
    }
    pub fn get_contents_count(&self) -> i32 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getContentsCount", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn is_empty(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "isEmpty", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }
    pub fn resolve_all(&self, path: &str) -> JavaStream<AbsoluteFileFolder> {
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
    pub fn resolve_file_by_id(&self, id: &str, deep: bool) -> AbsoluteFile {
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
    pub fn resolve_files(&self, path: &str) -> JavaStream<AbsoluteFile> {
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
    pub fn resolve_folder(&self, path: &str) -> AbsoluteFolder {
        let jvm = Jvm::attach_thread().unwrap();
        let path = InvocationArg::try_from(path).unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveFolder", &[path])
            .unwrap();
        AbsoluteFolder { instance }
    }
    pub fn resolve_folder_by_id(&self, id: &str) -> AbsoluteFolder {
        let jvm = Jvm::attach_thread().unwrap();
        let id = InvocationArg::try_from(id).unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveFolderById", &[id])
            .unwrap();
        AbsoluteFolder { instance }
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

impl AbsoluteFileFolderTrait for AbsoluteFolder {
    fn refreshed(&self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "refreshed", &[]).unwrap();
        AbsoluteFolder { instance }
    }
}
