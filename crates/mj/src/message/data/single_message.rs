use crate::message::data::message_source::MessageSource;
use crate::message::data::quote_reply::QuoteReply;

use crate::message::data::at::At;
use crate::message::data::at_all::AtAll;
use crate::message::data::dice::Dice;
use crate::message::data::face::Face;
use crate::message::data::file_message::FileMessage;
use crate::message::data::forward_message::ForwardMessage;
use crate::message::data::image::Image;
use crate::message::data::light_app::LightApp;
use crate::message::data::market_face::MarketFace;
use crate::message::data::music_share::MusicShare;
use crate::message::data::plain_text::PlainText;
use crate::message::data::poke_message::PokeMessage;
use crate::message::data::rock_paper_scissors::RockPaperScissors;
use crate::message::data::super_face::SuperFace;
use crate::message::data::unsupported_message::UnsupportedMessage;
use crate::message::data::vip_face::VipFace;
use j4rs::Instance;
use mjbase::env::GetEnvTrait;

// TODO: 需要知道 Java 或者 MessageChain 会不会返回除了以下消息之外的 SingleMessage
// TODO: 还有一些如 Audio 等消息没有实现，需要补上。
pub enum SingleMessage {
    At(At),
    AtAll(AtAll),
    Dice(Dice),
    Face(Face),
    FileMessage(FileMessage),
    ForwardMessage(ForwardMessage),
    Image(Image),
    LightApp(LightApp),
    MarketFace(MarketFace),
    MessageSource(MessageSource),
    MusicShare(MusicShare),
    PlainText(PlainText),
    PokeMessage(PokeMessage),
    QuoteReply(QuoteReply),
    RockPaperScissors(RockPaperScissors),
    SuperFace(SuperFace),
    UnsupportedMessage(UnsupportedMessage),
    VipFace(VipFace),
    // 以下这个应该不会被 MessageChain 返回吧？
}

impl GetEnvTrait for SingleMessage {
    fn get_instance(&self) -> Instance {
        match self {
            SingleMessage::At(a) => a.get_instance(),
            SingleMessage::AtAll(a) => a.get_instance(),
            SingleMessage::Dice(a) => a.get_instance(),
            SingleMessage::Face(a) => a.get_instance(),
            SingleMessage::FileMessage(a) => a.get_instance(),
            SingleMessage::ForwardMessage(a) => a.get_instance(),
            SingleMessage::Image(a) => a.get_instance(),
            SingleMessage::LightApp(a) => a.get_instance(),
            SingleMessage::MarketFace(a) => a.get_instance(),
            SingleMessage::MessageSource(a) => a.get_instance(),
            SingleMessage::MusicShare(a) => a.get_instance(),
            SingleMessage::PlainText(a) => a.get_instance(),
            SingleMessage::PokeMessage(a) => a.get_instance(),
            SingleMessage::QuoteReply(a) => a.get_instance(),
            SingleMessage::RockPaperScissors(a) => a.get_instance(),
            SingleMessage::SuperFace(a) => a.get_instance(),
            SingleMessage::UnsupportedMessage(a) => a.get_instance(),
            SingleMessage::VipFace(a) => a.get_instance(),
        }
    }
}
