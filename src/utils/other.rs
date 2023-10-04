pub mod enums {
    use crate::env::FromInstance;
    use j4rs::{Instance, Jvm};
    use num_enum::IntoPrimitive;

    pub enum HeartbeatStrategy {
        /// `HeartbeatStrategy.STAT_HB`
        S,
        /// `HeartbeatStrategy.REGISTER`
        R,
        /// `HeartbeatStrategy.NONE`
        N,
    }

    pub enum MiraiProtocol {
        /// `MiraiProtocol.ANDROID_PHONE`
        A,
        /// `MiraiProtocol.ANDROID_PAD`
        P,
        /// `MiraiProtocol.ANDROID_WATCH`
        W,
        /// `MiraiProtocol.IPAD`
        I,
        /// `MiraiProtocol.MACOS`
        M,
    }

    #[derive(IntoPrimitive)]
    #[repr(i32)]
    pub enum AvatarSpec {
        /// SMALLEST(40), 最高压缩等级。
        XS = 40,
        /// SMALL(41), 群员列表中的显示大小, 实际上是 40 px, 但会比 `SMALLEST` 好一些。
        S = 41,
        /// MEDIUM(100), 联系人列表中的显示大小。
        M = 100,
        /// LARGE(140), 消息列表中的显示大小。
        L = 140,
        /// LARGEST(640), 联系人详情页面中的显示大小。
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

    impl FromInstance for MemberMedalType {
        fn from_instance(instance: Instance) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.to_rust::<i32>(jvm.invoke(&instance, "getMask", &[]).unwrap())
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

    impl FromInstance for GroupHonorType {
        fn from_instance(instance: Instance) -> Self {
            let jvm = Jvm::attach_thread().unwrap();
            let id: i32 = jvm
                .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
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
