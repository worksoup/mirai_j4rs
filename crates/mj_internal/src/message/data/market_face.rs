use j4rs::Instance;
use mj_helper_macro::mj_all;

use crate::message::data::dice::Dice;
use crate::message::data::rock_paper_scissors::RockPaperScissors;
use crate::message::message_trait::{
    ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait, MessageTrait, SingleMessageTrait,
};
use crate::utils::backend::BotBackend;

#[mj_all("message.data.MarketFace")]
pub struct MarketFace<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageTrait<B> for MarketFace<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for MarketFace<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for MarketFace<B> {}

impl<B: BotBackend> MessageContentTrait<B> for MarketFace<B> {}

impl<B: BotBackend> MarketFaceTrait<B> for MarketFace<B> {}

#[mj_all("message.data.MarketFace")]
pub enum MarketFaceAll<B: BotBackend> {
    Dice(Dice<B>),
    #[fall]
    MarketFace(MarketFace<B>),
    RockPaperScissors(RockPaperScissors<B>),
}

impl<B: BotBackend> MessageTrait<B> for MarketFaceAll<B> {}

impl<B: BotBackend> SingleMessageTrait<B> for MarketFaceAll<B> {}

impl<B: BotBackend> ConstrainSingleTrait<B> for MarketFaceAll<B> {}

impl<B: BotBackend> MessageContentTrait<B> for MarketFaceAll<B> {}

impl<B: BotBackend> MarketFaceTrait<B> for MarketFaceAll<B> {}
