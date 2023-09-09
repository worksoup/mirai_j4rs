use contact_derive::GetInstanceDerive;
use j4rs::Instance;
use crate::env::GetEnvTrait;
use crate::message::message_trait::SingleMessageTrait;

pub trait MiraiRsCollectionTrait {
    type Element;
    fn get_size(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn contains(&self, element: &Self::Element) -> bool;
    fn contains_all(&self, elements: Self) -> bool;
}

pub trait MiraiRsIterableTrait: Iterator {}
