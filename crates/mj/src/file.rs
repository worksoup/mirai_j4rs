use crate::{contact::group::Group, message::data::file_message::FileMessage, utils::JavaStream};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::utils::primitive_byte_array_to_string;
use mj_base::{
    env::{FromInstance, GetClassTypeTrait, GetEnvTrait},
    utils::{external_resource_close, external_resource_from_file, is_instance_of},
};
use mj_macro::{java_type, FromInstanceDerive, GetInstanceDerive};

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
        let instance = self.get_instance();
        jvm.chain(&instance)
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
    fn get_uploader_id(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.chain(&self.get_instance())
            .unwrap()
            .invoke("getUploaderId", &[])
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
    fn to_file(&self) -> AbsoluteFile {
        let instance = self.get_instance();
        let instance = AbsoluteFile::cast_to_this_type(instance);
        AbsoluteFile::from_instance(instance)
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
    fn to_folder(&self) -> AbsoluteFolder {
        let instance = self.get_instance();
        let instance = AbsoluteFolder::cast_to_this_type(instance);
        AbsoluteFolder::from_instance(instance)
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

    /// 重命名，目前会失败。
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

#[derive(GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.contact.file.AbsoluteFile")]
pub struct AbsoluteFile {
    instance: Instance,
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
    ///
    /// 需要注意的是，目前已知原版 `Mirai-2.16.0` 存在 Bug, 返回的 MD5 不是固定的 16 字节。
    /// 所以此处以字符串形式返回。
    /// 该 Bug 大致原因是某些字节被额外转义了。
    /// 比如 `0x0a` 代表回车，被转移为了 `\n`, 即 `0x5c6e`, 这样结果就会多出一个字节。
    /// 已知部分转义情况：
    /// 0x00 -> 0x5c30 -- \0
    /// 0x0a -> 0x5c6e -- \n
    /// 0x0d -> 0x5c72 -- \r
    /// 0x1a -> 0x5c5a -- \Z
    /// 0x22 -> 0x5c22 -- \"
    /// 0x27 -> 0x5c27 -- \'
    /// 0x5c -> 0x5c5c -- \\
    pub fn get_md5(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let bytes = jvm.invoke(&self.instance, "getMd5", &[]).unwrap();
        let bytes = primitive_byte_array_to_string(&jvm, &bytes);
        jvm.to_rust(bytes).unwrap()
    }
    /// 文件内容 SHA-1.
    ///
    /// 需要注意的是，目前已知原版 `Mirai-2.16.0` 存在 Bug, 返回的 MD5 不是固定的 16 字节。
    /// 所以此处以字符串形式返回。
    /// 该 Bug 大致原因是某些字节被额外转义了。
    /// 比如 `0x0a` 代表回车，被转移为了 `\n`, 即 `0x5c6e`, 这样结果就会多出一个字节。
    /// 已知部分转义情况：
    /// 0x00 -> 0x5c30 -- \0
    /// 0x0a -> 0x5c6e -- \n
    /// 0x0d -> 0x5c72 -- \r
    /// 0x1a -> 0x5c5a -- \Z
    /// 0x22 -> 0x5c22 -- \"
    /// 0x27 -> 0x5c27 -- \'
    /// 0x5c -> 0x5c5c -- \\
    pub fn get_sha1(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        let bytes = jvm.invoke(&self.instance, "getSha1", &[]).unwrap();
        let bytes = primitive_byte_array_to_string(&jvm, &bytes);
        jvm.to_rust(bytes).unwrap()
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
        FileMessage::from_instance(instance)
    }
}

impl AbsoluteFileFolderTrait for AbsoluteFile {
    fn refreshed(&self) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "refreshed", &[]).unwrap();
        AbsoluteFile { instance }
    }
}

#[java_type("net.mamoe.mirai.contact.file.AbsoluteFileFolder")]
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
#[derive(GetInstanceDerive, FromInstanceDerive)]
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
        AbsoluteFile { instance }
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
    pub fn upload_new_file(&self, file_name: &str, path: &str) -> AbsoluteFile {
        let jvm = Jvm::attach_thread().unwrap();
        let res = external_resource_from_file(&jvm, path);
        let instance = jvm
            .invoke(
                &self.instance,
                "uploadNewFile",
                &[
                    InvocationArg::try_from(file_name).unwrap(),
                    InvocationArg::try_from(jvm.clone_instance(&res).unwrap()).unwrap(),
                ],
            )
            .unwrap();
        // Mirai 文档里说要 close.
        external_resource_close(&jvm, res);
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
        AbsoluteFolder { instance }
    }

    /// 上传新文件。
    pub fn upload_new_file(&self, path: &str) -> AbsoluteFile {
        let jvm = Jvm::attach_thread().unwrap();
        let res = external_resource_from_file(&jvm, path);
        let instance = jvm
            .invoke(
                &self.instance,
                "uploadNewFile",
                &[InvocationArg::try_from(jvm.clone_instance(&res).unwrap()).unwrap()],
            )
            .unwrap();
        // Mirai 文档里说要 close.
        external_resource_close(&jvm, res);
        AbsoluteFile { instance }
    }

    /// 上传新文件，传入的 callback 可以获取到当前上传文件的进度。
    pub fn upload_new_file_with_progression_callback() -> AbsoluteFile {
        todo!()
    }
}