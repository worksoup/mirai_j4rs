use std::{cmp::Ordering, marker::PhantomData, mem::transmute};

use j4rs::{Instance, InvocationArg, Jvm};

use mj_base::{data_wrapper::DataWrapper, env::FromInstanceTrait, utils::instance_from_i8_16};
use mj_macro::GetInstanceDerive;

pub struct Comparator<'a, T: FromInstanceTrait> {
    instance: Instance,
    internal_closure_raw: [i8; 16],
    __a: PhantomData<&'a ()>,
    _t: PhantomData<T>,
}

#[derive(GetInstanceDerive)]
pub struct ComparatorRaw {
    instance: Instance,
    internal_closure_raw: [i8; 16],
}

impl ComparatorRaw {
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}

impl<'a, T: FromInstanceTrait> Comparator<'a, T> {
    #[inline]
    fn internal_closure_as_i8_16<F>(f: &'a F) -> [i8; 16]
    where
        F: Fn(T, T) -> Ordering,
    {
        let call_from_java = Box::new(|value: DataWrapper<Instance>| -> Instance {
            let (val1, val2) = value.get::<DataWrapper<(T, T)>>().get_pair();
            let ordering = f(val1, val2);
            Instance::try_from(
                InvocationArg::try_from(unsafe { *{ &ordering as *const Ordering as *const i8 } })
                    .unwrap(),
            )
            .unwrap()
        });
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new<F>(closure: &'a F) -> Comparator<'a, T>
    where
        F: Fn(T, T) -> Ordering,
    {
        let jvm = Jvm::attach_thread().unwrap();
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(internal_closure_raw);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaComparator",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        Comparator {
            instance,
            internal_closure_raw,
            __a: PhantomData::default(),
            _t: PhantomData::default(),
        }
    }
    pub fn compare(&self, val1: InvocationArg, val2: InvocationArg) -> Ordering {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance, "compare", &[val1, val2])
            .unwrap();
        let cmp_result: i32 = jvm.to_rust(result).unwrap();
        cmp_result.cmp(&0)
    }
    pub fn to_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
    fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub(super) fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
    pub fn drop_and_to_raw(self) -> ComparatorRaw {
        let instance = self.instance;
        let internal_closure_raw = self.internal_closure_raw;
        ComparatorRaw {
            instance,
            internal_closure_raw,
        }
    }
}
