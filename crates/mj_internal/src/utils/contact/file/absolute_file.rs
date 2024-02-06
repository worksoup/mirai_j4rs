use crate::message::data::FileMessage;
use crate::utils::contact::file::{AbsoluteFileFolderTrait, AbsoluteFolder};
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::{
    env::{FromInstanceTrait, GetInstanceTrait},
    utils::primitive_byte_array_to_string,
};
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(GetInstanceDerive, AsInstanceDerive, FromInstanceDerive)]
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
        let bytes = primitive_byte_array_to_string(&jvm, bytes);
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
        let bytes = primitive_byte_array_to_string(&jvm, bytes);
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
