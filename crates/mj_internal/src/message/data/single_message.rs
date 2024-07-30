use mj_helper_macro::mj_all;

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
use crate::utils::backend::BotBackend;

// TODO: 需要知道 Java 或者 MessageChain 会不会返回除了以下消息之外的 SingleMessage
// TODO: 还有一些消息没有实现，需要补上。
#[mj_all("message.data.SingleMessage")]
pub enum SingleMessage<B: BotBackend> {
    At(At<B>),
    AtAll(AtAll<B>),
    Audio(Audio<B>),
    Face(Face<B>),
    FileMessage(FileMessage<B>),
    ForwardMessage(ForwardMessage<B>),
    Image(Image<B>),
    LightApp(LightApp<B>),
    MarketFaceAll(MarketFaceAll<B>),
    MessageSource(MessageSource<B>),
    MessageOrigin(MessageOrigin<B>),
    MusicShare(MusicShare<B>),
    PlainText(PlainText<B>),
    PokeMessage(PokeMessage<B>),
    QuoteReply(QuoteReply<B>),
    SuperFace(SuperFace<B>),
    VipFace(VipFace<B>),
    #[fall]
    UnsupportedMessage(UnsupportedMessage<B>),
    // 以下这个应该不会被 MessageChain 返回吧？
}

impl<B: BotBackend> MessageTrait<B> for SingleMessage<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for SingleMessage<B> {}
