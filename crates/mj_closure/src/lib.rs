//! 代表 Java 中的可调用对象。每个模块有一个结构体，命名规则是：
//!``` plain
//! LumiaAaBbCc <=> aa_bb_cc
//!      |             |
//!     Jvm          rust
//!```
//!
//! ## 使用方法
//! 以 [`consumer::Consumer`] 为例，通过 [`new`][consumer::Consumer::new] 方法可以获得一个结构体，传入参数是一个闭包的引用。
//!
//! 通过 [`to_instance`][consumer::Consumer::to_instance] 方法可以得到一个 `j4rs::Instance` 对象，
//! 该对象可作为 Jvm 侧可调用对象，在该结构体的生命周期内可以调用。
//!
//! 在内部使用请保存下该结构体实例，不要只保存其调用 [`to_instance`][consumer::Consumer::to_instance] 方法得到的 `j4rs::Instance` 对象。
//! 否则在实例 drop 之后的 Jvm 侧对可调用对象的调用操作将出错。
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(result_flattening)]

use j4rs::errors::J4RsError;
use j4rs::Instance;
use mj_base::data_wrapper::DataWrapper;
mod bi_function;
mod comparator;
mod consumer;
mod ffi_internal_test;
mod function;
mod interface;
mod kotlin;
mod predicate;
mod supplier;
mod utils;

pub use bi_function::*;
pub use comparator::*;
pub use consumer::*;
pub use function::*;
pub use interface::*;
pub use kotlin::*;
pub use predicate::*;
pub use supplier::*;

const POINTER_SIZE: usize =
    size_of::<*mut dyn Fn(DataWrapper<Instance>) -> Result<Instance, J4RsError>>();
type RawPointer = [i8; POINTER_SIZE];

// impl Drop for comparator::ComparatorRaw {
//     fn drop(&mut self) {
//         self.drop_internal_closure_raw()
//     }
// }
//
// impl Drop for consumer::ConsumerRaw {
//     fn drop(&mut self) {
//         self.drop_internal_closure_raw()
//     }
// }
//
// impl Drop for function::FunctionRaw {
//     fn drop(&mut self) {
//         self.drop_internal_closure_raw()
//     }
// }
//
// impl Drop for kt_func_0::KtFunc0Raw {
//     fn drop(&mut self) {
//         self.drop_internal_closure_raw()
//     }
// }
//
// impl Drop for kt_func_1::KtFunc1Raw {
//     fn drop(&mut self) {
//         self.drop_internal_closure_raw()
//     }
// }
//
// impl Drop for kt_func_2::KtFunc2Raw {
//     fn drop(&mut self) {
//         self.drop_internal_closure_raw()
//     }
// }
//
// impl Drop for predicate::PredicateRaw {
//     fn drop(&mut self) {
//         self.drop_internal_closure_raw()
//     }
// }
