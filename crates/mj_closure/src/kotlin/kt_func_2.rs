use std::{marker::PhantomData, mem::transmute};

use crate::utils::raw_pointer_to_instance;
use j4rs::{Instance, InvocationArg, Jvm};
use mj_base::{
    data_wrapper::DataWrapper,
    env::{TryFromInstanceTrait, GetInstanceTrait},
};
use mj_macro::GetInstanceDerive;

pub struct KtFunc2<'a, P1, P2, R>
where
    P1: TryFromInstanceTrait,
    P2: TryFromInstanceTrait,
    R: GetInstanceTrait + TryFromInstanceTrait,
{
    instance: Instance,
    internal_closure_raw: [i8; 16],
    _p1: PhantomData<P1>,
    _p2: PhantomData<P2>,
    __a: PhantomData<&'a ()>,
    _r: PhantomData<R>,
}

#[derive(GetInstanceDerive)]
pub struct KtFunc2Raw {
    instance: Instance,
    internal_closure_raw: [i8; 16],
}

impl KtFunc2Raw {
    pub fn get_internal_closure_raw(&self) -> *mut dyn Fn(DataWrapper<Instance>) -> Instance {
        unsafe { transmute(self.internal_closure_raw) }
    }
    pub fn drop_internal_closure_raw(&self) {
        let _boxed = unsafe { Box::from_raw(self.get_internal_closure_raw()) };
    }
}

impl<'a, P1, P2, R> KtFunc2<'a, P1, P2, R>
where
    P1: TryFromInstanceTrait,
    P2: TryFromInstanceTrait,
    R: GetInstanceTrait + TryFromInstanceTrait,
{
    #[inline]
    fn internal_closure_as_i8_16<F: Fn(P1, P2) -> R>(f: &'a F) -> [i8; 16] {
        let call_from_java = |value: DataWrapper<Instance>| -> Instance {
            let value = value.get::<DataWrapper<(P1, P2)>>();
            let (val1, val2) = value.unwrap().get_pair();
            f(val1, val2).get_instance().unwrap()
        };
        let call_from_java_raw: *mut dyn Fn(DataWrapper<Instance>) -> Instance =
            Box::into_raw(Box::new(call_from_java));
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new<F: Fn(P1, P2) -> R>(closure: &'a F) -> KtFunc2<'a, P1, P2, R> {
        let jvm = Jvm::attach_thread().unwrap();
        let internal_closure_raw = Self::internal_closure_as_i8_16(closure);
        let kt_func_2 = raw_pointer_to_instance::<"rt.lea.function.LumiaFunction">(internal_closure_raw);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaKtFunc2",
                &[InvocationArg::from(kt_func_2)],
            )
            .unwrap();
        KtFunc2 {
            instance,
            internal_closure_raw,
            _p1: PhantomData,
            _p2: PhantomData,
            __a: PhantomData,
            _r: PhantomData,
        }
    }
    pub fn to_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(&self.instance)
            .unwrap()
    }
    pub fn invoke(&self, val1: InvocationArg, val2: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm.invoke(&self.instance, "invoke", &[val1, val2]).unwrap();
        R::try_from_instance(result).unwrap()
    }
    pub fn drop_and_to_raw(self) -> KtFunc2Raw {
        let instance = self.instance;
        let internal_closure_raw = self.internal_closure_raw;
        KtFunc2Raw {
            instance,
            internal_closure_raw,
        }
    }
}