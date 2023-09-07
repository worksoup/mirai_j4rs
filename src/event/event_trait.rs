use j4rs::Instance;
use crate::env::{GetClassTypeTrait, GetEnvTrait};

pub trait MiraiEventTrait
    where
        Self: GetEnvTrait + GetClassTypeTrait,
{
    fn from_instance(instance: Instance) -> Self;
    fn cancel(&self);
    fn intercept(&self);
    fn is_canceled(&self) -> bool;
    fn is_intercepted(&self) -> bool;
    fn broadcast(&self);
}