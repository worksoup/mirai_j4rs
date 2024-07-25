use j4rs::errors::J4RsError;
use j4rs::{InvocationArg, Jvm};
use jbuchong::GetClassTypeTrait;

use crate::utils::other::enums::MiraiProtocol;

pub mod enums {
    use j4rs::errors::J4RsError;
    use j4rs::{Instance, InvocationArg, Jvm};
    use num_enum::IntoPrimitive;
    use std::fmt::{Display, Formatter};

    use jbuchong::java_type;
    use jbuchong::{GetInstanceTrait, TryFromInstanceTrait};

    use crate::utils::other::{protocol_enum_r2j, protocol_str2enum};

    pub enum HeartbeatStrategy {
        /// `HeartbeatStrategy.STAT_HB`
        #[doc(alias = "STAT_HB")]
        S,
        /// `HeartbeatStrategy.REGISTER`
        #[doc(alias = "REGISTER")]
        R,
        /// `HeartbeatStrategy.NONE`
        #[doc(alias = "NONE")]
        N,
    }

    #[derive(Debug)]
    #[java_type("net.mamoe.mirai.utils.BotConfiguration$MiraiProtocol")]
    pub enum MiraiProtocol {
        /// `MiraiProtocol.ANDROID_PHONE`
        #[doc(alias = "ANDROID_PHONE")]
        A,
        /// `MiraiProtocol.ANDROID_PAD`
        #[doc(alias = "ANDROID_PAD")]
        P,
        /// `MiraiProtocol.ANDROID_WATCH`
        #[doc(alias = "ANDROID_WATCH")]
        W,
        /// `MiraiProtocol.IPAD`
        #[doc(alias = "IPAD")]
        I,
        /// `MiraiProtocol.MACOS`
        #[doc(alias = "MACOS")]
        M,
    }

    impl GetInstanceTrait for MiraiProtocol {
        fn get_instance(&self) -> Result<Instance, J4RsError> {
            // let _ = Jvm::attach_thread().unwrap();
            Ok(protocol_enum_r2j(self).unwrap().instance().unwrap())
        }
    }

    impl TryFromInstanceTrait for MiraiProtocol {
        fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
            let jvm = Jvm::attach_thread().unwrap();
            let mp: String = jvm.to_rust(instance).unwrap();
            Ok(protocol_str2enum(mp))
        }
    }

    impl MiraiProtocol {
        pub fn is_nudge_supported(&self) -> bool {
            let jvm = Jvm::attach_thread().unwrap();
            let b = jvm
                .invoke(
                    &self.get_instance().unwrap(),
                    "isNudgeSupported",
                    InvocationArg::empty(),
                )
                .unwrap();
            jvm.to_rust(b).unwrap()
        }
        pub fn is_qr_login_supported(&self) -> bool {
            let jvm = Jvm::attach_thread().unwrap();
            let b = jvm
                .invoke(
                    &self.get_instance().unwrap(),
                    "isQRLoginSupported",
                    InvocationArg::empty(),
                )
                .unwrap();
            jvm.to_rust(b).unwrap()
        }
    }

    #[java_type("net.mamoe.mirai.contact.AvatarSpec")]
    #[derive(IntoPrimitive)]
    #[repr(i32)]
    pub enum AvatarSpec {
        /// SMALLEST(40), 最高压缩等级。
        #[doc(alias = "SMALLEST")]
        XS = 40,
        /// SMALL(41), 群员列表中的显示大小, 实际上是 40 px, 但会比 [`SMALLEST`](AvatarSpec::XS) 好一些。
        #[doc(alias = "SMALL")]
        S = 41,
        /// MEDIUM(100), 联系人列表中的显示大小。
        #[doc(alias = "MEDIUM")]
        M = 100,
        /// LARGE(140), 消息列表中的显示大小。
        #[doc(alias = "LARGE")]
        L = 140,
        /// LARGEST(640), 联系人详情页面中的显示大小。
        #[doc(alias = "LARGEST")]
        XL = 640,
        /// ORIGINAL(0), 原图。
        ORIGINAL = 0,
    }

    #[derive(num_enum::FromPrimitive, IntoPrimitive, Debug, PartialEq, Eq, Hash)]
    #[repr(i32)]
    pub enum MemberMedalType {
        #[default]
        OWNER = 300,
        ADMIN = 301,
        SPECIAL = 302,
        ACTIVE = 315,
    }

    impl TryFromInstanceTrait for MemberMedalType {
        fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.to_rust::<i32>(
                jvm.invoke(&instance, "getMask", InvocationArg::empty())
                    .unwrap(),
            )
            .map(i32::into)
        }
    }

    #[derive(num_enum::FromPrimitive, IntoPrimitive, Debug, Hash, Eq, PartialEq)]
    #[repr(i32)]
    pub enum GroupHonorType {
        #[default]
        Talkative = 1,
        Performer = 2,
        Legend = 3,
        StrongNewbie = 4,
        Emotion = 5,
        Bronze = 6,
        Silver = 7,
        Golden = 8,
        Whirlwind = 9,
        Richer = 10,
        RedPacket = 11,
    }

    impl GroupHonorType {
        fn internal_to_i32(a: &GroupHonorType) -> i32 {
            unsafe { *(a as *const GroupHonorType as *const i32) }
        }
        pub fn hash_code(&self) -> i32 {
            Self::internal_to_i32(self)
        }
    }
    impl Display for GroupHonorType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(
                ("GroupHonorType(id=".to_string()
                    + Self::internal_to_i32(self).to_string().as_str()
                    + ")")
                    .as_str(),
            )
        }
    }

    impl TryFromInstanceTrait for GroupHonorType {
        fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.to_rust(
                jvm.invoke(&instance, "getId", InvocationArg::empty())
                    .unwrap(),
            )
            .map(i32::into)
        }
    }
}

#[inline]
pub fn protocol_enum_r2j(protocol: &MiraiProtocol) -> Result<InvocationArg, J4RsError> {
    let jvm = Jvm::attach_thread().unwrap();
    Ok(InvocationArg::from(jvm.field(
        &jvm.static_class(<MiraiProtocol as GetClassTypeTrait>::get_type_name())?,
        match protocol {
            MiraiProtocol::A => "ANDROID_PHONE",
            MiraiProtocol::I => "IPAD",
            MiraiProtocol::M => "MACOS",
            MiraiProtocol::P => "ANDROID_PAD",
            MiraiProtocol::W => "ANDROID_WATCH",
        },
    )?))
}

#[inline]
pub fn protocol_str2enum(protocol: String) -> MiraiProtocol {
    match protocol.as_str() {
        "ANDROID_PHONE" => MiraiProtocol::A,
        "IPAD" => MiraiProtocol::I,
        "MACOS" => MiraiProtocol::M,
        "ANDROID_PAD" => MiraiProtocol::P,
        "ANDROID_WATCH" => MiraiProtocol::W,
        _ => {
            eprintln!("协议枚举转换失败，默认转换结果为安卓协议。");
            MiraiProtocol::A
        }
    }
}
