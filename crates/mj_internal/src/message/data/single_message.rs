use crate::message::{
    data::{
        at::At, at_all::AtAll, audio::Audio, face::Face, file_message::FileMessage,
        forward_message::ForwardMessage, image::Image, light_app::LightApp,
        market_face::MarketFaceAll, message_source::MessageSource, music_share::MusicShare,
        plain_text::PlainText, poke_message::PokeMessage, quote_reply::QuoteReply,
        super_face::SuperFace, unsupported_message::UnsupportedMessage, vip_face::VipFace,
    },
    message_trait::{MessageTrait, SingleMessageTrait},
};
use j4rs::Instance;
use mj_base::env::{AsInstanceTrait, FromInstance, GetClassTypeTrait, GetInstanceTrait};
use mj_macro::java_type;

// TODO: 需要知道 Java 或者 MessageChain 会不会返回除了以下消息之外的 SingleMessage
// TODO: 还有一些如 Audio 等消息没有实现，需要补上。
#[java_type("net.mamoe.mirai.message.data.SingleMessage")]
pub enum SingleMessage {
    At(At),
    AtAll(AtAll),
    Audio(Audio),
    Face(Face),
    FileMessage(FileMessage),
    ForwardMessage(ForwardMessage),
    Image(Image),
    LightApp(LightApp),
    MarketFaceAll(MarketFaceAll),
    MessageSource(MessageSource),
    MusicShare(MusicShare),
    PlainText(PlainText),
    PokeMessage(PokeMessage),
    QuoteReply(QuoteReply),
    SuperFace(SuperFace),
    VipFace(VipFace),
    UnsupportedMessage(UnsupportedMessage),
    // 以下这个应该不会被 MessageChain 返回吧？
}

impl GetInstanceTrait for SingleMessage {
    fn get_instance(&self) -> Instance {
        macro_rules! get_branch {
            ($($e:ident),*) => {
                match self {
                    $(
                    SingleMessage::$e(a) => a.get_instance(),
                    )*
                    SingleMessage::UnsupportedMessage(a) => a.get_instance()
                }
            };
        }
        // 注意没有 `UnsupportedMessage`.
        get_branch!(
            At,
            AtAll,
            Audio,
            Face,
            FileMessage,
            ForwardMessage,
            Image,
            LightApp,
            MarketFaceAll,
            MessageSource,
            MusicShare,
            PlainText,
            PokeMessage,
            QuoteReply,
            SuperFace,
            VipFace
        )
    }
}

impl AsInstanceTrait for SingleMessage {
    fn as_instance(&self) -> &Instance {
        macro_rules! as_branch {
            ($($e:ident),*) => {
                match self {
                    $(
                    SingleMessage::$e(a) => a.as_instance(),
                    )*
                    SingleMessage::UnsupportedMessage(a) => a.as_instance()
                }
            };
        }
        // 注意没有 `UnsupportedMessage`.
        as_branch!(
            At,
            AtAll,
            Audio,
            Face,
            FileMessage,
            ForwardMessage,
            Image,
            LightApp,
            MarketFaceAll,
            MessageSource,
            MusicShare,
            PlainText,
            PokeMessage,
            QuoteReply,
            SuperFace,
            VipFace
        )
    }
}

impl FromInstance for SingleMessage {
    fn from_instance(instance: Instance) -> Self {
        macro_rules! from_branch {
            ($($e:ident),*) => {
                $(if $e::is_this_type(&instance){
                    SingleMessage::$e($e::from_instance($e::cast_to_this_type(instance)))
                } else)*{
                    SingleMessage::UnsupportedMessage(UnsupportedMessage::from_instance(instance))
                }
            };
        }
        // 注意没有 `UnsupportedMessage`.
        from_branch!(
            At,
            AtAll,
            Audio,
            Face,
            FileMessage,
            ForwardMessage,
            Image,
            LightApp,
            MarketFaceAll,
            MessageSource,
            MusicShare,
            PlainText,
            PokeMessage,
            QuoteReply,
            SuperFace,
            VipFace
        )
    }
}

impl MessageTrait for SingleMessage {}

impl SingleMessageTrait for SingleMessage {}
