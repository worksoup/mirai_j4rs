use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::java_type;
use jbuchong::{GetClassTypeTrait, GetInstanceTrait};

#[java_type("net.mamoe.mirai.auth.BotAuthorization")]
pub enum BotAuthorization {
    Password(String),
    Md5([u8; 16]),
    QrCode,
}

// TODO: 测试是否可以直接直接转换。
impl GetInstanceTrait for BotAuthorization {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        match self {
            BotAuthorization::Password(password) => Ok(jvm
                .invoke_static(
                    <Self as GetClassTypeTrait>::get_type_name(),
                    "byPassword",
                    &[InvocationArg::try_from(password).unwrap()],
                )
                .unwrap()),
            BotAuthorization::Md5(md5) => {
                let password_md5 = md5.map(|e| {
                    InvocationArg::try_from(e as i8)
                        .unwrap()
                        .into_primitive()
                        .unwrap()
                });
                let arg = jvm.create_java_array("byte", &password_md5).unwrap();
                let arg = InvocationArg::from(arg);
                Ok(jvm
                    .invoke_static(
                        <Self as GetClassTypeTrait>::get_type_name(),
                        "byPassword",
                        &[arg],
                    )
                    .unwrap())
            }
            BotAuthorization::QrCode => Ok(jvm
                .invoke_static(
                    <Self as GetClassTypeTrait>::get_type_name(),
                    "byQRCode",
                    InvocationArg::empty(),
                )
                .unwrap()),
        }
    }
}
