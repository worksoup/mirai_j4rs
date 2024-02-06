pub use bot::*;
pub use event::*;
pub use friend::*;
pub use group::*;
pub use message::*;
pub use other_client::*;
pub use stranger::*;
pub use user::*;

mod bot;
mod event;
mod friend;
mod group;
mod message;
mod other_client;
mod stranger;
mod user;

pub trait BroadcastControllableTrait: MiraiEventTrait {}
pub trait CancellableEventTrait: MiraiEventTrait {}
