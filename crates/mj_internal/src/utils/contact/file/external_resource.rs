use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::env::GetClassTypeTrait;
use mj_macro::mj_all;

#[inline]
fn i8_array_to_java_byte_array(vec: &[i8]) -> Instance {
    let jvm = Jvm::attach_thread().unwrap();
    let args: Vec<_> = vec
        .iter()
        .map(|elem| {
            InvocationArg::try_from(elem)
                .unwrap()
                .into_primitive()
                .unwrap()
        })
        .collect();
    jvm.create_java_array("byte", &args).unwrap()
}

// TODO: getMD5, etc.
/// 一个不可变的外部资源。仅包含资源内容、大小、文件类型、校验值而不包含文件名、文件位置等。
///
/// 请注意要在使用完后手动 [close](ExternalResource::close).
/// 具体说明请参考 Mirai 文档。
#[mj_all("utils.ExternalResource")]
pub struct ExternalResource {
    instance: Instance,
}
impl ExternalResource {
    /// 从文件创建 [`ExternalResource`].
    ///
    /// 该函数的 IO 操作是通过 `java.io.File` 完成的。所以没有额外的复制。
    /// 但是指示路径的字符串需要遵守 java 的格式。
    pub fn create_from_file(path: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke_static(
                Self::get_type_name(),
                "create",
                &[InvocationArg::try_from(
                    jvm.create_instance("java.io.File", &[InvocationArg::try_from(path).unwrap()])
                        .unwrap(),
                )
                .unwrap()],
            )
            .unwrap();
        Self { instance }
    }
    /// 从字节数组创建 [`ExternalResource`].
    ///
    /// 请注意该函数会把数据从 rust 世界复制到 jvm 世界，这会造成额外开销。
    pub fn create_from_bytes(bytes: &[i8]) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let bytes = i8_array_to_java_byte_array(bytes);
        let instance = jvm
            .invoke_static(
                Self::get_type_name(),
                "create",
                &[InvocationArg::try_from(bytes).unwrap()],
            )
            .unwrap();
        Self { instance }
    }
    /// 从文件创建 [`ExternalResource`], 同时指定文件格式。
    ///
    /// 格式默认会从文件头识别, 支持的文件类型:
    /// - 图片类型: png, jpg, gif, tif, bmp
    /// - 语音类型: amr, silk
    /// - 视频类类型: mp4, mkv
    ///
    /// 该函数的 IO 操作是通过 `java.io.File` 完成的。所以没有额外的复制。
    /// 但是指示路径的字符串需要遵守 java 的格式。
    pub fn create_from_file_(path: &str, format_name: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let format_name = InvocationArg::try_from(format_name).unwrap();
        let instance = jvm
            .invoke_static(
                Self::get_type_name(),
                "create",
                &[
                    InvocationArg::try_from(
                        jvm.create_instance(
                            "java.io.File",
                            &[InvocationArg::try_from(path).unwrap()],
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                    format_name,
                ],
            )
            .unwrap();
        Self { instance }
    }
    /// 从字节数组创建 [`ExternalResource`], 同时指定文件格式。
    ///
    /// 格式默认会从文件头识别, 支持的文件类型:
    /// - 图片类型: png, jpg, gif, tif, bmp
    /// - 语音类型: amr, silk
    /// - 视频类类型: mp4, mkv
    ///
    /// 请注意该函数会把数据从 rust 世界复制到 jvm 世界，这会造成额外开销。
    pub fn create_from_bytes_(bytes: &[i8], format_name: &str) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let format_name = InvocationArg::try_from(format_name).unwrap();
        let bytes = i8_array_to_java_byte_array(bytes);
        let instance = jvm
            .invoke_static(
                Self::get_type_name(),
                "create",
                &[InvocationArg::try_from(bytes).unwrap(), format_name],
            )
            .unwrap();
        Self { instance }
    }

    pub fn close(self) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm.invoke(&self.instance, "close", &[]);
    }
}
