pub mod enums {
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
}
