#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(specialization)]

pub use prefix::MIRAI_PREFIX;

pub mod data_wrapper;
pub mod env;
mod prefix;
pub mod types;
pub mod utils;
