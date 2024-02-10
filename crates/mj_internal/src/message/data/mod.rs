pub use at::*;
pub use at_all::*;
pub use audio::*;
pub use dice::*;
pub use face::*;
pub use file_message::*;
pub use forward_message::*;
pub use image::*;
pub use light_app::*;
pub use market_face::*;
pub use message_chain::*;
pub use message_origin::*;
pub use message_source::*;
pub use music_share::*;
pub use plain_text::*;
pub use poke_message::*;
pub use quote_reply::*;
pub use rock_paper_scissors::*;
pub use single_message::*;
pub use super_face::*;
pub use unsupported_message::*;
pub use vip_face::*;

mod at;
mod at_all;
mod audio;
mod dice;
mod face;
mod file_message;
mod forward_message;
mod image;
mod light_app;
mod market_face;
mod message_chain;
mod message_origin;
mod message_source;
mod music_share;
mod plain_text;
mod poke_message;
mod quote_reply;
mod rock_paper_scissors;
mod single_message;
mod super_face;
mod unsupported_message;
mod vip_face;

// use j4rs::Instance;
// use mj_macro::{FromInstanceDerive, GetInstanceDerive};
// use crate::message::message_trait::{CodableMessageTrait, ConstrainSingleTrait, MessageContentTrait, MessageTrait, SingleMessageTrait};
//
// #[derive(GetInstanceDerive,FromInstanceDerive)]
// pub struct  {
//     instance: Instance,
// }
// impl MessageTrait for  {}
// impl SingleMessageTrait for  {}
// impl MessageContentTrait for  {}
// impl ConstrainSingleTrait for  {}
// impl CodableMessageTrait for  {}
