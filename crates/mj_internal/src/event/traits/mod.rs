mod bot;
mod event;
mod friend;
mod group;
mod message;
mod stranger;
mod user;

pub use bot::*;
pub use event::*;
pub use friend::*;
pub use group::*;
pub use message::*;
pub use stranger::*;
pub use user::*;

pub trait BroadcastControllableTrait: MiraiEventTrait {}
pub trait CancellableEventTrait: MiraiEventTrait {}
