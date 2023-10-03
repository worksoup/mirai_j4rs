use crate::{
    contact::group::Group,
    env::{FromInstance, GetEnvTrait},
    message::FileMessage,
    utils::internal::is_instance_of,
    utils::FileFolderStream,
};
use contact_derive::GetInstanceDerive;
use j4rs::{Instance, InvocationArg, Jvm};

pub trait AbsoluteFileFolderTrait: Sized + GetEnvTrait {
    fn delete(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("delete", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn exists(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("exists", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_absolute_path(&self) -> String // 这里应该是远程文件，所以先不用 PathBuf
    {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getAbsolutePath", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    // FileSupported 当前只有 Group
    fn get_contact(&self) -> Group {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "getContact", &[]).unwrap();
        Group::from_instance(instance)
    }
    fn get_extension<T: AbsoluteFileFolderTrait>(file_or_folder: T) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&file_or_folder.get_instance())
            .unwrap()
            .invoke("getExtension", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_id(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getId", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_last_modified_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getLastModifiedTime", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_name(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getName", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_name_without_extension<T: AbsoluteFileFolderTrait>(file_or_folder: T) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&file_or_folder.get_instance())
            .unwrap()
            .invoke("getNameWithoutExtension", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_parent(&self) -> AbsoluteFolder {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.get_instance(), "getParent", &[]).unwrap();
        AbsoluteFolder { instance }
    }
    fn get_upload_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getUploadTime", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn get_upload_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getUploadId", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn is_file(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("isFile", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn is_folder(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("isFolder", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn refresh(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("refresh", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn refreshed(&self) -> Self;
    fn rename_to(&self, name: &str) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("renameTo", &[InvocationArg::try_from(name).unwrap()])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    fn to_string(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("toString", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
}

#[derive(GetInstanceDerive)]
pub struct AbsoluteFile {
    pub(crate) instance: Instance,
}

impl FromInstance for AbsoluteFile {
    fn from_instance(instance: Instance) -> Self {
        AbsoluteFile { instance }
    }
}

impl AbsoluteFile {
    /// 文件到期时间戳，单位为秒。
    pub fn get_expiry_time(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("getExpiryTime", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    /// 文件内容 MD5.
    pub fn get_md5(&self) -> [i8; 16] {
        let jvm = Jvm::attach_thread().unwrap();
        crate::utils::internal::get_bytes_md5_and_cast_to_i8_16(jvm, &self.instance)
    }
    /// 文件内容 SHA-1. 我记着是 20 位来着，记着测试。TODO: 测试一下。
    pub fn get_sha1(&self) -> [i8; 20] {
        let jvm = Jvm::attach_thread().unwrap();
        let bytes = jvm.invoke(&self.instance, "getSha1", &[]).unwrap();
        let instance = jvm
            .invoke_static(
                "org.apache.commons.lang3.ArrayUtils",
                "toObject",
                &[InvocationArg::try_from(bytes).unwrap()],
            )
            .unwrap();
        let instance = jvm
            .invoke_static(
                "java.util.Array",
                "stream",
                &[InvocationArg::try_from(instance).unwrap()],
            )
            .unwrap();
        jvm.chain(&instance)
            .unwrap()
            .invoke("toList", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    /// 文件大小 (占用空间), 单位 byte.
    pub fn get_size(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("getSize", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn get_url(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("getUrl", &[])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    pub fn move_to(&self, remote_folder: &AbsoluteFolder) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let folder = InvocationArg::try_from(remote_folder.get_instance()).unwrap();
        jvm.chain(&self.instance)
            .unwrap()
            .invoke("moveTo", &[folder])
            .unwrap()
            .to_rust()
            .unwrap()
    }
    /// 得到 AbsoluteFile 所对应的 FileMessage.
    /// 注: 在 上传文件 时就已经发送了文件消息, FileMessage 不可手动发送
    pub fn to_message(&self) -> FileMessage {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "refreshed", &[]).unwrap();
        FileMessage { instance }
    }
}

impl AbsoluteFileFolderTrait for AbsoluteFile {
    fn refreshed(&self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "refreshed", &[]).unwrap();
        AbsoluteFile { instance }
    }
}

pub enum AbsoluteFileFolder {
    AbsoluteFile(AbsoluteFile),
    AbsoluteFolder(AbsoluteFolder),
}

impl FromInstance for AbsoluteFileFolder {
    fn from_instance(instance: Instance) -> Self {
        if is_instance_of(&instance, "net.mamoe.mirai.contact.file.AbsoluteFile") {
            AbsoluteFileFolder::AbsoluteFile(AbsoluteFile { instance })
        } else {
            AbsoluteFileFolder::AbsoluteFolder(AbsoluteFolder { instance })
        }
    }
}

impl GetEnvTrait for AbsoluteFileFolder {
    fn get_instance(&self) -> Instance {
        match self {
            AbsoluteFileFolder::AbsoluteFile(a) => a.get_instance(),
            AbsoluteFileFolder::AbsoluteFolder(a) => a.get_instance(),
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

/// # 绝对目录标识。
/// 精确表示一个远程目录。不会受同名文件或目录的影响。
/// Mirai 中有些方法会返回 Flow 或 Stream, 后者的方法名称有 Stream 后缀，
/// 这里包装的全部都是 Stream 版本，哪怕没有后缀。这些方法会返回一个迭代器，以此模拟其操作。
#[derive(GetInstanceDerive)]
pub struct AbsoluteFolder {
    pub(crate) instance: Instance,
}

impl FromInstance for AbsoluteFolder {
    fn from_instance(instance: Instance) -> Self {
        AbsoluteFolder { instance }
    }
}

impl AbsoluteFolder {
    pub fn children(&self) -> FileFolderStream<AbsoluteFileFolder> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "childrenStream", &[]).unwrap();
        FileFolderStream {
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
    pub fn files(&self) -> FileFolderStream<AbsoluteFile> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "filesStream", &[]).unwrap();
        FileFolderStream {
            instance,
            _unused: Default::default(),
        }
    }
    pub fn folders(&self) -> FileFolderStream<AbsoluteFolder> {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "foldersStream", &[]).unwrap();
        FileFolderStream {
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
    pub fn resolve_all(&self, path: &str) -> FileFolderStream<AbsoluteFileFolder> {
        let jvm = Jvm::attach_thread().unwrap();
        let path = InvocationArg::try_from(path).unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveAllStream", &[path])
            .unwrap();
        FileFolderStream {
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
        AbsoluteFile { instance }
    }
    pub fn resolve_files(&self, path: &str) -> FileFolderStream<AbsoluteFile> {
        let jvm = Jvm::attach_thread().unwrap();
        let path = InvocationArg::try_from(path).unwrap();
        let instance = jvm
            .invoke(&self.instance, "resolveFilesStream", &[path])
            .unwrap();
        FileFolderStream {
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
    pub fn upload_new_file(&self, path: &str) -> AbsoluteFile {
        let jvm = Jvm::attach_thread().unwrap();
        let res = jvm
            .invoke_static(
                "net.mamoe.mirai.utils.ExternalResource",
                "create",
                &[InvocationArg::try_from(
                    jvm.create_instance("java.io.File", &[InvocationArg::try_from(path).unwrap()])
                        .unwrap(),
                )
                    .unwrap()],
            )
            .unwrap();
        let instance = jvm
            .invoke(
                &self.instance,
                "uploadNewFile",
                &[InvocationArg::try_from(jvm.clone_instance(&res).unwrap()).unwrap()],
            )
            .unwrap();
        // Mirai 文档里说要 close.
        let _ = jvm.invoke(&res, "close", &[]);
        AbsoluteFile { instance }
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

#[derive(GetInstanceDerive)]
pub struct RemoteFiles {
    pub(crate) instance: Instance,
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
        AbsoluteFolder { instance }
    }

    /// 上传新文件。
    pub fn upload_new_file(&self, path: &str) -> AbsoluteFile {
        let jvm = Jvm::attach_thread().unwrap();
        let res = jvm
            .invoke_static(
                "net.mamoe.mirai.utils.ExternalResource",
                "create",
                &[InvocationArg::try_from(
                    jvm.create_instance("java.io.File", &[InvocationArg::try_from(path).unwrap()])
                        .unwrap(),
                )
                    .unwrap()],
            )
            .unwrap();
        let instance = jvm
            .invoke(
                &self.instance,
                "uploadNewFile",
                &[InvocationArg::try_from(jvm.clone_instance(&res).unwrap()).unwrap()],
            )
            .unwrap();
        // Mirai 文档里说要 close.
        let _ = jvm.invoke(&res, "close", &[]);
        AbsoluteFile { instance }
    }

    /// 上传新文件，传入的 callback 可以获取到当前上传文件的进度。
    pub fn upload_new_file_with_progression_callback() -> AbsoluteFile {
        todo!()
    }
}
