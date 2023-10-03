pub mod ffi;
mod ffi_internal_test;
pub(crate) mod internal;
pub mod other;

use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::ffi::{Comparator, Consumer, Function, Predicate};
use crate::utils::internal::instance_is_null;
use j4rs::{Instance, InvocationArg, Jvm};
use std::{cmp::Ordering, marker::PhantomData};

pub trait MiraiRsCollectionTrait {
    type Element;
    fn get_size(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn contains(&self, element: &Self::Element) -> bool;
    fn contains_all(&self, elements: Self) -> bool;
}

pub trait MiraiRsIterableTrait: Iterator {}

/// 对应 Stream<AbsoluteFileFolder>
pub struct FileFolderStream<T: FromInstance> {
    pub(crate) instance: Instance,
    pub(crate) _unused: PhantomData<T>,
}

impl<T: FromInstance> GetEnvTrait for FileFolderStream<T> {
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
}

impl<T: FromInstance> FromInstance for FileFolderStream<T> {
    fn from_instance(instance: Instance) -> Self {
        FileFolderStream {
            instance,
            _unused: PhantomData::default(),
        }
    }
}

impl<T: FromInstance> FileFolderStream<T> {
    pub fn sorted_array_by<F>(&self, compare: F) -> Vec<T>
        where
            F: FnMut(&T, &T) -> Ordering,
    {
        let mut array = self.to_vec();
        array.sort_by(compare);
        array
    }
    pub fn filter<P>(&self, p: P) -> FileFolderStream<T>
        where
            P: Fn(T) -> bool,
            T: FromInstance,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let p = Predicate::new(p);
        let predicate = InvocationArg::try_from(p.get_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "filter", &[predicate]).unwrap();
        drop(p);
        FileFolderStream::from_instance(instance)
    }

    pub fn map<B: FromInstance, F>(&self, f: F) -> FileFolderStream<B>
        where
            F: Fn(T) -> B,
            T: FromInstance,
            B: GetEnvTrait + FromInstance,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Function::new(f);
        let mapper = InvocationArg::try_from(f.get_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "map", &[mapper]).unwrap();
        drop(f);
        FileFolderStream::from_instance(instance)
    }

    pub fn for_each<F>(&self, f: F)
        where
            F: Fn(T),
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Consumer::new(f);
        let predicate = InvocationArg::try_from(f.get_instance()).unwrap();
        let _ = jvm.invoke(&self.instance, "filter", &[predicate]).unwrap();
        drop(f);
    }

    pub fn count(&self) -> i64 {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "count", &[]).unwrap();
        jvm.to_rust(instance).unwrap()
    }

    pub fn flat_map<U: FromInstance, F>(&self, f: F) -> FileFolderStream<U>
        where
            F: Fn(T) -> FileFolderStream<U>,
            T: FromInstance,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Function::new(f);
        let mapper = InvocationArg::try_from(f.get_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "flatMap", &[mapper]).unwrap();
        drop(f);
        FileFolderStream::from_instance(instance)
    }

    pub fn skip(&self, n: i64) -> FileFolderStream<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let n = InvocationArg::try_from(n)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm.invoke(&self.instance, "skip", &[n]).unwrap();
        FileFolderStream::from_instance(instance)
    }

    pub fn limit(&self, max_size: i64) -> FileFolderStream<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let max_size = InvocationArg::try_from(max_size)
            .unwrap()
            .into_primitive()
            .unwrap();
        let instance = jvm.invoke(&self.instance, "limit", &[max_size]).unwrap();
        FileFolderStream::from_instance(instance)
    }

    pub fn max_by<F>(&self, f: F) -> Option<T>
        where
            F: Fn(&T, &T) -> Ordering + 'static,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Comparator::new(f);
        let compare = InvocationArg::try_from(f.get_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "max", &[compare]).unwrap();
        drop(f);
        if instance_is_null(&instance) {
            Some(T::from_instance(instance))
        } else {
            None
        }
    }

    pub fn min_by<F>(&self, f: F) -> Option<T>
        where
            F: Fn(&T, &T) -> Ordering + 'static,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let f = Comparator::new(f);
        let compare = InvocationArg::try_from(f.get_instance()).unwrap();
        let instance = jvm.invoke(&self.instance, "min", &[compare]).unwrap();
        drop(f);
        if instance_is_null(&instance) {
            Some(T::from_instance(instance))
        } else {
            None
        }
    }
    pub fn to_vec(&self) -> Vec<T> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut array = Vec::new();
        let instance = jvm.invoke(&self.instance, "toList", &[]).unwrap();
        loop {
            let has_next: bool = jvm
                .to_rust(jvm.invoke(&instance, "hasNxt", &[]).unwrap())
                .unwrap();
            if has_next {
                let next = jvm.invoke(&instance, "next", &[]).unwrap();
                array.push(T::from_instance(next))
            } else {
                break;
            }
        }
        array
    }
}
