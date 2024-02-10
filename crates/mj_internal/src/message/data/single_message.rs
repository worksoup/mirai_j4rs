use mj_macro::mj_all;

use crate::message::data::message_origin::MessageOrigin;
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

// TODO: 需要知道 Java 或者 MessageChain 会不会返回除了以下消息之外的 SingleMessage
// TODO: 还有一些消息没有实现，需要补上。
#[mj_all("message.data.SingleMessage")]
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
    MessageOrigin(MessageOrigin),
    MusicShare(MusicShare),
    PlainText(PlainText),
    PokeMessage(PokeMessage),
    QuoteReply(QuoteReply),
    SuperFace(SuperFace),
    VipFace(VipFace),
    #[fall]
    UnsupportedMessage(UnsupportedMessage),
    // 以下这个应该不会被 MessageChain 返回吧？
}

impl MessageTrait for SingleMessage {}

impl SingleMessageTrait for SingleMessage {}
