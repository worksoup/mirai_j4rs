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

pub mod comparator;
pub mod consumer;
pub mod function;
pub mod predicate;
pub mod kt_func_0;
pub mod kt_func_1;
pub mod kt_func_2;


use crate::env::{FromInstance, GetEnvTrait};

impl<'a, T: FromInstance> Drop for comparator::Comparator<'a, T> {
    fn drop(&mut self) {
        self.drop_internal_closure_raw()
    }
}

impl<'a, T: FromInstance> Drop for consumer::Consumer<'a, T> {
    fn drop(&mut self) {
        self.drop_internal_closure_raw()
    }
}

impl<'a, T: FromInstance, R: GetEnvTrait + FromInstance> Drop for function::Function<'a, T, R> {
    fn drop(&mut self) {
        self.drop_internal_closure_raw()
    }
}

impl<'a, R: GetEnvTrait + FromInstance> Drop for kt_func_0::KtFunc0<'a, R> {
    fn drop(&mut self) {
        self.drop_internal_closure_raw()
    }
}

impl<'a, T: FromInstance, R: GetEnvTrait + FromInstance> Drop for kt_func_1::KtFunc1<'a, T, R> {
    fn drop(&mut self) {
        self.drop_internal_closure_raw()
    }
}

impl<'a, P1: FromInstance, P2: FromInstance, R: GetEnvTrait + FromInstance> Drop for kt_func_2::KtFunc2<'a, P1, P2, R> {
    fn drop(&mut self) {
        self.drop_internal_closure_raw()
    }
}

impl<'a, T: FromInstance> Drop for predicate::Predicate<'a, T> {
    fn drop(&mut self) {
        self.drop_internal_closure_raw()
    }
}