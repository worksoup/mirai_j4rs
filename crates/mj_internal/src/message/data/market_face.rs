use j4rs::Instance;

use mj_base::env::{AsInstanceTrait, GetClassTypeTrait};
use mj_base::env::{FromInstanceTrait, GetInstanceTrait};
use mj_macro::mj_all;

use crate::message::data::dice::Dice;
use crate::message::data::rock_paper_scissors::RockPaperScissors;
use crate::message::message_trait::{
    ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait, MessageTrait, SingleMessageTrait,
};

#[mj_all("message.data.MarketFace")]
pub struct MarketFace {
    instance: Instance,
}

impl MessageTrait for MarketFace {}

impl SingleMessageTrait for MarketFace {}

impl ConstrainSingleTrait for MarketFace {}

impl MessageContentTrait for MarketFace {}

impl MarketFaceTrait for MarketFace {}

#[mj_all("message.data.MarketFace")]
pub enum MarketFaceAll {
    Dice(Dice),
    #[fall]
    MarketFace(MarketFace),
    RockPaperScissors(RockPaperScissors),
}

impl MessageTrait for MarketFaceAll {}

impl SingleMessageTrait for MarketFaceAll {}

impl ConstrainSingleTrait for MarketFaceAll {}

impl MessageContentTrait for MarketFaceAll {}

impl MarketFaceTrait for MarketFaceAll {}
