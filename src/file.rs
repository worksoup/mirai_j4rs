use crate::contact::group::Group;
use crate::env::GetEnvTrait;
use contact_derive::GetInstanceDerive;
use j4rs::{Instance, InvocationArg, Jvm};
use crate::message::FileMessage;

pub trait AbsoluteFileFolder: Sized + GetEnvTrait {
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
    fn get_contact(&self) /*-> impl FileSupportedTrait */
    {
        todo!()
    }
    fn get_extension<T: AbsoluteFileFolder>(file_or_folder: T) -> String {
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
    fn get_name_without_extension<T: AbsoluteFileFolder>(file_or_folder: T) -> String {
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

impl AbsoluteFile {
    pub fn get_expiry_time(&self) -> i64 { todo!() }
    pub fn get_md5(&self) -> [i8; 16] { todo!() }
    pub fn get_sha1(&self) -> [i8; 16] { todo!() }
    pub fn get_size(&self) -> i64 { todo!() }
    pub fn get_url(&self) -> String { todo!() }
    pub fn move_to(&self, remote_path: &AbsoluteFolder) -> bool { todo!() }
    pub fn to_message(&self) -> FileMessage { todo!() }
}

impl AbsoluteFileFolder for AbsoluteFile {
    fn refreshed(&self) -> Self {
        todo!()
    }
}

#[derive(GetInstanceDerive)]
pub struct AbsoluteFolder {
    pub(crate) instance: Instance,
}

impl AbsoluteFolder {}

impl AbsoluteFileFolder for AbsoluteFolder {
    fn refreshed(&self) -> Self {
        todo!()
    }
}

#[derive(GetInstanceDerive)]
pub struct RemoteFiles {
    pub(crate) instance: Instance,
}

impl RemoteFiles {
    /// 该函数返回 FileSupported, 但是目前应该只有 Group 的吧？.
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
    pub fn upload_new_file() { todo!() }

    /// 上传新文件，传入的 callback 可以获取到当前上传文件的进度。
    pub fn upload_new_file_with_progression_callback() { todo!() }
}
