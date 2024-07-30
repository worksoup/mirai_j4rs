use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{
    java_all,
    utils::instance_is_null,
    Comparator, Consumer, FromInstanceTrait, Function, GetClassTypeTrait, Predicate,
    {GetInstanceTrait, TryFromInstanceTrait},
};
use std::{cmp::Ordering, marker::PhantomData};

pub mod backend;
pub mod bot_builder;
mod bot_configuration;
pub mod contact;
pub mod data_wrapper;
mod device_info;
pub mod just_for_examples;
pub mod login_solver;
mod mirai_list;
mod mirai_logger;
mod mirai_map;
pub mod other;

pub use bot_configuration::*;
pub use device_info::*;
pub use mirai_list::*;
pub use mirai_logger::*;
pub use mirai_map::*;

pub trait MiraiRsCollectionTrait {
    type Element;
    fn get_size(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn contains(&self, element: &Self::Element) -> bool;
    fn contains_all(&self, elements: Self) -> bool;
}

pub trait MiraiRsIterableTrait: Iterator {}

/// 对应 `Stream<AbsoluteFileFolder>`
#[java_all]
pub struct JavaStream<T: TryFromInstanceTrait + GetClassTypeTrait> {
    pub(crate) instance: Instance,
    pub(crate) _unused: PhantomData<T>,
}

impl<T: TryFromInstanceTrait + GetClassTypeTrait> JavaStream<T> {
    pub fn sorted_array_by<F>(&self, compare: F) -> Vec<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut array = self.to_vec();
        array.sort_by(compare);
        array
    }
    pub fn filter<P>(&self, p: P) -> JavaStream<T>
    where
        P: Fn(T) -> bool + 'static,
        T: TryFromInstanceTrait,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let p = Predicate::new(p);
        let predicate = InvocationArg::try_from(p.get_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "filter", &[predicate]).unwrap();
        p.drop();
        JavaStream::from_instance(instance)
    }

    pub fn map<B, F>(&self, f: F) -> JavaStream<B>
    where
        F: Fn(T) -> B + 'static,
        T: TryFromInstanceTrait,
        B: GetClassTypeTrait + TryFromInstanceTrait + GetInstanceTrait,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Function::new(f);
        let mapper = InvocationArg::from(f.get_instance().unwrap());
        let instance = jvm.invoke(&self.instance, "map", &[mapper]).unwrap();
        f.drop();
        JavaStream::from_instance(instance)
    }

    pub fn for_each<F>(&self, f: F)
    where
        F: Fn(T) + 'static,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Consumer::new(f);
        let predicate = InvocationArg::from(f.get_instance().unwrap());
        let _ = jvm.invoke(&self.instance, "filter", &[predicate]).unwrap();
        f.drop();
    }

    pub fn count(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "count", InvocationArg::empty())
            .unwrap();
        jvm.to_rust(instance).unwrap()
    }

    pub fn flat_map<U: TryFromInstanceTrait + GetClassTypeTrait, F>(&self, f: F) -> JavaStream<U>
    where
        F: Fn(T) -> JavaStream<U> + 'static,
        T: TryFromInstanceTrait,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Function::new(f);
        let mapper = InvocationArg::from(f.get_instance().unwrap());
        let instance = jvm.invoke(&self.instance, "flatMap", &[mapper]).unwrap();
        f.drop();
        JavaStream::from_instance(instance)
    }

    pub fn skip(&self, n: i64) -> JavaStream<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let n = InvocationArg::try_from(n)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm.invoke(&self.instance, "skip", &[n]).unwrap();
        JavaStream::from_instance(instance)
    }

    pub fn limit(&self, max_size: i64) -> JavaStream<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let max_size = InvocationArg::try_from(max_size)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm.invoke(&self.instance, "limit", &[max_size]).unwrap();
        JavaStream::from_instance(instance)
    }

    pub fn max_by<F>(&self, f: F) -> Option<T>
    where
        F: Fn(T, T) -> Ordering + 'static,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Comparator::new(f);
        let compare = InvocationArg::from(f.get_instance().unwrap());
        let instance = jvm.invoke(&self.instance, "max", &[compare]).unwrap();
        f.drop();
        if !instance_is_null(&instance) {
            T::try_from_instance(instance).ok()
        } else {
            None
        }
    }

    pub fn min_by<F>(&self, f: F) -> Option<T>
    where
        F: Fn(T, T) -> Ordering + 'static,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Comparator::new(f);
        let compare = InvocationArg::from(f.get_instance().unwrap());
        let instance = jvm.invoke(&self.instance, "min", &[compare]).unwrap();
        f.drop();
        if !instance_is_null(&instance) {
            Some(T::try_from_instance(instance).unwrap())
        } else {
            None
        }
    }
    pub fn to_vec(&self) -> Vec<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut array = Vec::new();
        let instance = jvm
            .invoke(&self.instance, "toList", InvocationArg::empty())
            .unwrap();
        let instance = jvm
            .invoke(&instance, "iterator", InvocationArg::empty())
            .unwrap();
        loop {
            let has_next: bool = jvm
                .to_rust(
                    jvm.invoke(&instance, "hasNext", InvocationArg::empty())
                        .unwrap(),
                )
                .unwrap();
            if has_next {
                let next = jvm
                    .invoke(&instance, "next", InvocationArg::empty())
                    .unwrap();
                if let Ok(e) = T::try_from_instance(T::cast_to_this_type(next)) {
                    array.push(e)
                }
            } else {
                break;
            }
        }
        array
    }
}
