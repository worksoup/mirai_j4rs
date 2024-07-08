use j4rs::errors::J4RsError;
use j4rs::Instance;

use jbuchong::{AsInstanceTrait, GetClassTypeTrait};
use jbuchong::{TryFromInstanceTrait, GetInstanceTrait};
use mj_helper_macro::mj_all;

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
