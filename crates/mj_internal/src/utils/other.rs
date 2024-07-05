use j4rs::{InvocationArg, Jvm};

use mj_base::env::GetClassTypeTrait;

use crate::utils::other::enums::MiraiProtocol;

pub mod enums {
    use j4rs::{Instance, InvocationArg, Jvm};
    use num_enum::IntoPrimitive;

    use mj_base::env::{FromInstanceTrait, GetInstanceTrait};
    use mj_macro::java_type;

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
    #[java_type("utils.BotConfiguration$MiraiProtocol")]
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
        fn get_instance(&self) -> Instance {
            // let _ = Jvm::attach_thread().unwrap();
            protocol_enum_r2j(self).unwrap().instance().unwrap()
        }
    }

    impl FromInstanceTrait for MiraiProtocol {
        fn from_instance(instance: Instance) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            let mp: String = jvm.to_rust(instance).unwrap();
            protocol_str2enum(mp)
        }
    }

    impl MiraiProtocol {
        pub fn is_nudge_supported(&self) -> bool {
            let jvm = Jvm::attach_thread().unwrap();
            let b = jvm
                .invoke(
                    &self.get_instance(),
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
                    &self.get_instance(),
                    "isQRLoginSupported",
                    InvocationArg::empty(),
                )
                .unwrap();
            jvm.to_rust(b).unwrap()
        }
    }

    #[java_type("contact.AvatarSpec")]
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

    #[derive(num_enum::FromPrimitive, IntoPrimitive, Debug, Hash)]
    #[repr(i32)]
    pub enum MemberMedalType {
        #[default]
        OWNER = 300,
        ADMIN = 301,
        SPECIAL = 302,
        ACTIVE = 315,
    }

    impl PartialEq<Self> for MemberMedalType {
        fn eq(&self, other: &Self) -> bool {
            (unsafe { *(self as *const Self as *const i32) })
                .eq(&unsafe { *(other as *const Self as *const i32) })
        }
    }

    impl Eq for MemberMedalType {}

    impl FromInstanceTrait for MemberMedalType {
        fn from_instance(instance: Instance) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.to_rust::<i32>(
                jvm.invoke(&instance, "getMask", InvocationArg::empty())
                    .unwrap(),
            )
            .unwrap()
            .into()
        }
    }

    #[derive(num_enum::FromPrimitive, IntoPrimitive, Debug, Hash)]
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
        pub fn to_string(&self) -> String {
            "GroupHonorType(id=".to_string()
                + Self::internal_to_i32(self).to_string().as_str()
                + ")"
        }
        pub fn hash_code(&self) -> i32 {
            Self::internal_to_i32(self)
        }
    }

    impl FromInstanceTrait for GroupHonorType {
        fn from_instance(instance: Instance) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            let id: i32 = jvm
                .to_rust(
                    jvm.invoke(&instance, "getId", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap();
            id.into()
        }
    }

    impl PartialEq for GroupHonorType {
        fn eq(&self, other: &Self) -> bool {
            let a = GroupHonorType::internal_to_i32(self);
            let b = GroupHonorType::internal_to_i32(other);
            a.eq(&b)
        }
    }

    impl Eq for GroupHonorType {}
}

#[inline]
pub fn protocol_enum_r2j(
    protocol: &MiraiProtocol,
) -> Result<InvocationArg, std::convert::Infallible> {
    InvocationArg::try_from(
        Jvm::attach_thread()
            .unwrap()
            .field(
                &Jvm::attach_thread()
                    .unwrap()
                    .static_class(<MiraiProtocol as GetClassTypeTrait>::get_type_name().as_str())
                    .unwrap(),
                match protocol {
                    MiraiProtocol::A => "ANDROID_PHONE",
                    MiraiProtocol::I => "IPAD",
                    MiraiProtocol::M => "MACOS",
                    MiraiProtocol::P => "ANDROID_PAD",
                    MiraiProtocol::W => "ANDROID_WATCH",
                },
            )
            .unwrap(),
    )
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
