use crate::message::data::dice::Dice;
use crate::message::data::rock_paper_scissors::RockPaperScissors;
use crate::message::message_trait::{
    ConstrainSingleTrait, MarketFaceTrait, MessageContentTrait, MessageTrait, SingleMessageTrait,
};
use j4rs::Instance;
use mj_base::env::{AsInstanceTrait, GetClassTypeTrait};
use mj_base::env::{FromInstance, GetInstanceTrait};
use mj_macro::{java_type, AsInstanceDerive, FromInstanceDerive, GetInstanceDerive};

#[derive(AsInstanceDerive, GetInstanceDerive, FromInstanceDerive)]
#[java_type("net.mamoe.mirai.message.data.MarketFace")]
pub struct MarketFace {
    instance: Instance,
}

impl MessageTrait for MarketFace {}

impl SingleMessageTrait for MarketFace {}

impl ConstrainSingleTrait for MarketFace {}

impl MessageContentTrait for MarketFace {}

impl MarketFaceTrait for MarketFace {}

#[java_type("net.mamoe.mirai.message.data.MarketFace")]
pub enum MarketFaceAll {
    Dice(Dice),
    MarketFace(MarketFace),
    RockPaperScissors(RockPaperScissors),
}

impl GetInstanceTrait for MarketFaceAll {
    fn get_instance(&self) -> Instance {
        match self {
            MarketFaceAll::Dice(a) => a.get_instance(),
            MarketFaceAll::MarketFace(a) => a.get_instance(),
            MarketFaceAll::RockPaperScissors(a) => a.get_instance(),
        }
    }
}

impl AsInstanceTrait for MarketFaceAll {
    fn as_instance(&self) -> &Instance {
        match self {
            MarketFaceAll::Dice(a) => a.as_instance(),
            MarketFaceAll::MarketFace(a) => a.as_instance(),
            MarketFaceAll::RockPaperScissors(a) => a.as_instance(),
        }
    }
}

impl FromInstance for MarketFaceAll {
    fn from_instance(instance: Instance) -> Self {
        if Dice::is_this_type(&instance) {
            MarketFaceAll::Dice(Dice::from_instance(Dice::cast_to_this_type(instance)))
        } else if RockPaperScissors::is_this_type(&instance) {
            MarketFaceAll::RockPaperScissors(RockPaperScissors::from_instance(
                RockPaperScissors::cast_to_this_type(instance),
            ))
        } else {
            MarketFaceAll::MarketFace(MarketFace::from_instance(MarketFace::cast_to_this_type(
                instance,
            )))
        }
    }
}

impl MessageTrait for MarketFaceAll {}

impl SingleMessageTrait for MarketFaceAll {}

impl ConstrainSingleTrait for MarketFaceAll {}

impl MessageContentTrait for MarketFaceAll {}

impl MarketFaceTrait for MarketFaceAll {}
