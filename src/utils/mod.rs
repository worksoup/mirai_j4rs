pub mod ffi;
pub(crate) mod internal;
mod ffi_internal_test;
pub mod other;

use crate::env::{FromInstance, GetEnvTrait};
use j4rs::{Instance, Jvm};
use std::cmp::Ordering;
use std::marker::PhantomData;

pub trait MiraiRsCollectionTrait {
    type Element;
    fn get_size(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn contains(&self, element: &Self::Element) -> bool;
    fn contains_all(&self, elements: Self) -> bool;
}

pub trait MiraiRsIterableTrait: Iterator {}

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

/// TODO: 暂时先用 to_vec 凑合一下吧。
impl<T: FromInstance> FileFolderStream<T> {
    pub fn sorted_array_by<F>(&self, compare: F) -> Vec<T>
        where
            F: FnMut(&T, &T) -> Ordering,
    {
        let mut array = self.to_vec();
        array.sort_by(compare);
        array
    }
    // 我觉得我可以直接假定没有重复文件。
    // pub fn distinct(&self) {
    //     ()
    // }
    pub fn close(self) {
        todo!()
    }
    pub fn filter<P>(&self, predicate: P) -> FileFolderStream<T>
        where
            P: FnMut(&T) -> bool,
    {
        todo!()
    }

    pub fn map<B: FromInstance, F>(&self, f: F) -> FileFolderStream<B>
        where
            F: FnMut(T) -> B,
    {
        todo!()
    }

    pub fn for_each<F>(&self, f: F)
        where
            F: FnMut(T),
    {
        todo!()
    }

    pub fn count(&self) -> i64 {
        todo!()
    }
    pub fn collect<B: FromIterator<T>>(&self) -> B {
        todo!()
    }
    pub fn find_first<P>(&self, predicate: P) -> Option<T>
        where
            P: FnMut(&T) -> bool,
    {
        todo!()
    }
    pub fn find_any<P>(&self, predicate: P) -> Option<T>
        where
            P: FnMut(&T) -> bool,
    {
        todo!()
    }

    pub fn flat_map<U: FromInstance, F>(&self, f: F) -> FileFolderStream<U>
        where
            F: FnMut(T) -> FileFolderStream<U>,
    {
        todo!()
    }

    pub fn skip(&self, n: i64) -> FileFolderStream<T> {
        todo!()
    }

    pub fn limit(&self, max_size: i64) -> FileFolderStream<T> {
        todo!()
    }

    pub fn max_by<F>(&self, compare: F) -> Option<T>
        where
            F: FnMut(&T, &T) -> Ordering,
    {
        todo!()
    }

    pub fn min_by<F>(&self, compare: F) -> Option<T>
        where
            F: FnMut(&T, &T) -> Ordering,
    {
        todo!()
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