use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::internal::instance_from_i8_16;
use contact_derive::GetInstanceDerive;
use j4rs::{errors::J4RsError, prelude::*, Instance, InvocationArg, Jvm};
use j4rs_derive::*;
use std::{cmp::Ordering, marker::PhantomData, mem::transmute, pin::Pin};

#[derive(GetInstanceDerive)]
pub struct InstanceWrapper {
    instance: Instance,
}

impl InstanceWrapper {
    pub fn get<E>(&self) -> E
        where
            E: FromInstance,
    {
        E::from_instance(
            Jvm::attach_thread()
                .unwrap()
                .clone_instance(&self.instance)
                .unwrap(),
        )
    }
}

impl FromInstance for InstanceWrapper {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

#[call_from_java("rt.lea.LumiaConsumer.nativeAccept")]
fn lumia_consumer_accept(consumer_as_i8_16: Instance, arg: Instance) {
    let consumer_raw: [i8; 16] = Jvm::attach_thread()
        .unwrap()
        .to_rust(consumer_as_i8_16)
        .unwrap();
    println!(
        "lumia_consumer_accept, in {}, {}:{}",
        file! {},
        line!(),
        column!()
    );
    println!("consumer_raw: {:?}", consumer_raw);
    let consumer: *mut dyn Fn(InstanceWrapper) -> () = unsafe { transmute(consumer_raw) };
    unsafe {
        let _ = (*consumer)(InstanceWrapper::from_instance(arg));
    };
}

pub(crate) struct Consumer<T, F>
    where
        T: FromInstance,
        F: Fn(T) -> (),
{
    closure: F,
    instance: Option<Instance>,
    internal_closure_raw: Option<[i8; 16]>,
    _unused: PhantomData<T>,
}

impl<T, F> GetEnvTrait for Consumer<T, F>
    where
        T: FromInstance,
        F: Fn(T) -> (),
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(self.instance.as_ref().unwrap())
            .unwrap()
    }
}

impl<T, F: Fn(T) -> ()> Consumer<T, F>
    where
        T: FromInstance,
{
    pub fn new(closure: F) -> Pin<Box<Consumer<T, F>>> {
        let mut consumer: Consumer<T, F> = Consumer {
            closure,
            instance: None,
            internal_closure_raw: None,
            _unused: Default::default(),
        };
        let closure_ref = &consumer.closure;
        let call_from_java = Box::new(|value: InstanceWrapper| {
            let value = value.get::<T>();
            closure_ref(value);
        });
        let call_from_java_raw: *mut dyn Fn(InstanceWrapper) = Box::into_raw(call_from_java);
        let call_from_java_raw_as_i8_16 = unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) };
        consumer.internal_closure_raw = Some(call_from_java_raw_as_i8_16);
        println!("closure_to_consumer");
        println!("{:?}", call_from_java_raw_as_i8_16);
        let jvm = Jvm::attach_thread().unwrap();
        let call_from_java_raw_as_java_bytes =
            crate::utils::internal::i8_16_to_bytes_16(&jvm, call_from_java_raw_as_i8_16);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaConsumer",
                &[InvocationArg::try_from(call_from_java_raw_as_java_bytes).unwrap()],
            )
            .unwrap();
        consumer.instance = Some(instance);
        Box::pin(consumer)
    }
    pub fn accept(&self, arg: InvocationArg) {
        let jvm = Jvm::attach_thread().unwrap();
        let _ = jvm
            .invoke(&self.instance.as_ref().unwrap(), "accept", &[arg])
            .unwrap();
    }
}

impl<T, F: Fn(T) -> ()> Drop for Consumer<T, F>
    where
        T: FromInstance,
{
    fn drop(&mut self) {
        let consumer: *mut dyn Fn(InstanceWrapper) -> () =
            unsafe { transmute(self.internal_closure_raw.unwrap()) };
        self.internal_closure_raw = None;
        let boxed = unsafe { Box::from_raw(consumer) };
        drop(boxed)
    }
}

struct KtPairWrapper<P1, P2>
    where
        P1: FromInstance,
        P2: FromInstance,
{
    val1: P1,
    val2: P2,
}

impl<P1, P2> KtPairWrapper<P1, P2>
    where
        P1: FromInstance,
        P2: FromInstance,
{
    pub fn get_pair(self) -> (P1, P2) {
        (self.val1, self.val2)
    }
}

impl<P1, P2> FromInstance for KtPairWrapper<P1, P2>
    where
        P1: FromInstance,
        P2: FromInstance,
{
    fn from_instance(instance: Instance) -> Self {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.cast(&instance, "kotlin.Pair").unwrap();
        let val1 = jvm.invoke(&instance, "getFirst", &[]).unwrap();
        let val2 = jvm.invoke(&instance, "getSecond", &[]).unwrap();
        let val1 = P1::from_instance(val1);
        let val2 = P2::from_instance(val2);
        Self { val1, val2 }
    }
}

pub(crate) struct Comparator<T, F>
    where
        T: FromInstance,
        F: Fn(&T, &T) -> Ordering,
{
    closure: F,
    instance: Option<Instance>,
    internal_closure_raw: Option<[i8; 16]>,
    _t: PhantomData<T>,
}

impl<T, F> GetEnvTrait for Comparator<T, F>
    where
        T: FromInstance,
        F: Fn(&T, &T) -> Ordering,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(self.instance.as_ref().unwrap())
            .unwrap()
    }
}

impl<T, F> Comparator<T, F>
    where
        T: FromInstance,
        F: Fn(&T, &T) -> Ordering + 'static,
{
    pub fn new(closure: F) -> Pin<Box<Comparator<T, F>>> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut comparator: Comparator<T, F> = Comparator {
            closure,
            instance: None,
            internal_closure_raw: None,
            _t: Default::default(),
        };
        let closure_ref = &comparator.closure;
        let call_from_java = Box::new(|value: InstanceWrapper| -> Instance {
            let value = value.get::<KtPairWrapper<T, T>>();
            let (val1, val2) = value.get_pair();
            let ordering = closure_ref(&val1, &val2);
            match ordering {
                Ordering::Less => Instance::try_from(InvocationArg::try_from(-1).unwrap()).unwrap(),
                Ordering::Equal => Instance::try_from(InvocationArg::try_from(0).unwrap()).unwrap(),
                Ordering::Greater => {
                    Instance::try_from(InvocationArg::try_from(1).unwrap()).unwrap()
                }
            }
        });
        let call_from_java_raw: *mut dyn Fn(InstanceWrapper) -> Instance =
            Box::into_raw(call_from_java);
        let call_from_java_as_i8_16 = unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) };
        comparator.internal_closure_raw = Some(call_from_java_as_i8_16);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(call_from_java_as_i8_16);
        let lumia_comparator = jvm
            .create_instance(
                "rt.lea.LumiaComparator",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        comparator.instance = Some(lumia_comparator);
        Box::pin(comparator)
    }
    pub fn compare(&self, val1: InvocationArg, val2: InvocationArg) -> Ordering {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance.as_ref().unwrap(), "compare", &[val1, val2])
            .unwrap();
        let cmp_result: i32 = jvm.to_rust(result).unwrap();
        cmp_result.cmp(&0)
    }
}

#[call_from_java("rt.lea.LumiaFunction.nativeApply")]
fn lumia_function_apply(
    function_raw_as_i8_16: Instance,
    val1: Instance,
) -> Result<Instance, String> {
    let function_raw: [i8; 16] = Jvm::attach_thread()
        .unwrap()
        .to_rust(function_raw_as_i8_16)
        .unwrap();
    println!(
        "lumia_function_apply, in {}, {}:{}",
        file! {},
        line!(),
        column!()
    );
    println!("function_raw: {:?}", function_raw);
    let function: *mut dyn Fn(InstanceWrapper) -> Instance = unsafe { transmute(function_raw) };
    let value = unsafe { (*function)(InstanceWrapper::from_instance(val1)) };
    Ok(value)
}

pub(crate) struct Function<T, F, R>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    closure: F,
    instance: Option<Instance>,
    internal_closure_raw: Option<[i8; 16]>,
    _t: PhantomData<T>,
    _r: PhantomData<R>,
}

impl<T, F, R> GetEnvTrait for Function<T, F, R>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(self.instance.as_ref().unwrap()).unwrap()
    }
}

impl<T, F, R> Function<T, F, R>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    pub(crate) fn internal_closure_as_i8_16(closure_ref: &F) -> [i8; 16] {
        let call_from_java = Box::new(|value: InstanceWrapper| -> Instance {
            let value = value.get::<T>();
            let value = closure_ref(value);
            value.get_instance()
        });
        let call_from_java_raw: *mut dyn Fn(InstanceWrapper) -> Instance =
            Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    pub fn new(closure: F) -> Pin<Box<Function<T, F, R>>> {
        let mut function: Function<T, F, R> = Function {
            closure,
            instance: None,
            internal_closure_raw: None,
            _t: Default::default(),
            _r: Default::default(),
        };
        let closure_ref = &function.closure;
        let call_from_java_raw_as_i8_16 = Self::internal_closure_as_i8_16(closure_ref);
        function.internal_closure_raw = Some(call_from_java_raw_as_i8_16);
        println!("closure_to_function\n{:?}", call_from_java_raw_as_i8_16);
        let instance = instance_from_i8_16::<"rt.lea.LumiaFunction">(call_from_java_raw_as_i8_16);
        function.instance = Some(instance);
        Box::pin(function)
    }
    pub fn apply(&self, arg: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance.as_ref().unwrap(), "apply", &[arg])
            .unwrap();
        R::from_instance(result)
    }
}

impl<T, F, R> Drop for Function<T, F, R>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    fn drop(&mut self) {
        let function: *mut dyn Fn(InstanceWrapper) -> Instance =
            unsafe { transmute(self.internal_closure_raw.unwrap()) };
        self.internal_closure_raw = None;
        let boxed = unsafe { Box::from_raw(function) };
        drop(boxed)
    }
}

#[call_from_java("rt.lea.LumiaPredicate.nativeTest")]
fn lumia_predicate_test(
    predicate_raw_as_i8_16: Instance,
    item: Instance,
) -> Result<Instance, String> {
    let predicate_raw: [i8; 16] = Jvm::attach_thread()
        .unwrap()
        .to_rust(predicate_raw_as_i8_16)
        .unwrap();
    println!(
        "lumia_predicate_test, in {}, {}:{}",
        file! {},
        line!(),
        column!()
    );
    println!("predicate_raw: {:?}", predicate_raw);
    let function: *mut dyn Fn(InstanceWrapper) -> Result<InvocationArg, J4RsError> =
        unsafe { transmute(predicate_raw) };
    let value = unsafe { (*function)(InstanceWrapper::from_instance(item)) }
        .map_err(|error| format!("{}", error))?;
    Instance::try_from(value).map_err(|error| format!("{}", error))
}

pub(crate) struct Predicate<T, F>
    where
        T: FromInstance,
        F: Fn(T) -> bool,
{
    closure: F,
    instance: Option<Instance>,
    internal_closure_raw: Option<[i8; 16]>,
    _t: PhantomData<T>,
}

impl<T, F> GetEnvTrait for Predicate<T, F>
    where
        T: FromInstance,
        F: Fn(T) -> bool,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(self.instance.as_ref().unwrap())
            .unwrap()
    }
}

impl<T, F> Predicate<T, F>
    where
        T: FromInstance,
        F: Fn(T) -> bool,
{
    pub fn new(closure: F) -> Pin<Box<Predicate<T, F>>> {
        let mut predicate: Predicate<T, F> = Predicate {
            closure,
            instance: None,
            internal_closure_raw: None,
            _t: Default::default(),
        };
        let closure_ref = &predicate.closure;
        let call_from_java = Box::new(
            |value: InstanceWrapper| -> Result<InvocationArg, J4RsError> {
                let value = value.get::<T>();
                let value = closure_ref(value);
                InvocationArg::try_from(value)
            },
        );
        let call_from_java_raw: *mut dyn Fn(InstanceWrapper) -> Result<InvocationArg, J4RsError> =
            Box::into_raw(call_from_java);
        let call_from_java_raw_as_i8_16 = unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) };
        predicate.internal_closure_raw = Some(call_from_java_raw_as_i8_16);
        println!("closure_to_predicate\n{:?}", call_from_java_raw_as_i8_16);
        let jvm = Jvm::attach_thread().unwrap();
        let call_from_java_raw_as_java_bytes =
            crate::utils::internal::i8_16_to_bytes_16(&jvm, call_from_java_raw_as_i8_16);
        let instance = jvm
            .create_instance(
                "rt.lea.LumiaPredicate",
                &[InvocationArg::try_from(call_from_java_raw_as_java_bytes).unwrap()],
            )
            .unwrap();
        predicate.instance = Some(instance);
        Box::pin(predicate)
    }
    pub fn test(&self, arg: InvocationArg) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance.as_ref().unwrap(), "test", &[arg])
            .unwrap();
        jvm.to_rust(result).unwrap()
    }
}

impl<T, F> Drop for Predicate<T, F>
    where
        T: FromInstance,
        F: Fn(T) -> bool,
{
    fn drop(&mut self) {
        let predicate: *mut dyn Fn(InstanceWrapper) -> Result<InvocationArg, J4RsError> =
            unsafe { transmute(self.internal_closure_raw.unwrap()) };
        self.internal_closure_raw = None;
        let boxed = unsafe { Box::from_raw(predicate) };
        drop(boxed)
    }
}

pub(crate) struct KtFunc0<R, F>
    where
        F: Fn() -> R,
        R: GetEnvTrait + FromInstance,
{
    closure: F,
    instance: Option<Instance>,
    internal_closure_raw: Option<[i8; 16]>,
}

impl<R, F> GetEnvTrait for KtFunc0<R, F>
    where
        F: Fn() -> R,
        R: GetEnvTrait + FromInstance,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(self.instance.as_ref().unwrap())
            .unwrap()
    }
}

impl<R, F> KtFunc0<R, F>
    where
        F: Fn() -> R,
        R: GetEnvTrait + FromInstance,
{
    pub fn new(closure: F) -> Pin<Box<KtFunc0<R, F>>> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut kt_func0: KtFunc0<R, F> = KtFunc0 {
            closure,
            instance: None,
            internal_closure_raw: None,
        };
        let closure_ref = &kt_func0.closure;
        let call_from_java =
            Box::new(|value: InstanceWrapper| -> Instance { closure_ref().get_instance() });
        let call_from_java_raw: *mut dyn Fn(InstanceWrapper) -> Instance =
            Box::into_raw(call_from_java);
        let call_from_java_as_i8_16 = unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) };
        kt_func0.internal_closure_raw = Some(call_from_java_as_i8_16);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(call_from_java_as_i8_16);
        let lumia_comparator = jvm
            .create_instance(
                "rt.lea.LumiaKtFunc0",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        kt_func0.instance = Some(lumia_comparator);
        Box::pin(kt_func0)
    }
    pub fn invoke(&self) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance.as_ref().unwrap(), "invoke", &[])
            .unwrap();
        R::from_instance(result)
    }
}

pub(crate) struct KtFunc1<T, R, F>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    closure: F,
    instance: Option<Instance>,
    internal_closure_raw: Option<[i8; 16]>,
    _t: PhantomData<T>,
}

impl<T, R, F> GetEnvTrait for KtFunc1<T, R, F>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(self.instance.as_ref().unwrap())
            .unwrap()
    }
}

impl<T, R, F> KtFunc1<T, R, F>
    where
        T: FromInstance,
        F: Fn(T) -> R,
        R: GetEnvTrait + FromInstance,
{
    pub fn new(closure: F) -> Pin<Box<KtFunc1<T, R, F>>> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut kt_func1: KtFunc1<T, R, F> = KtFunc1 {
            closure,
            instance: None,
            internal_closure_raw: None,
            _t: Default::default(),
        };
        let closure_ref = &kt_func1.closure;
        let call_from_java = Box::new(|value: InstanceWrapper| -> Instance {
            closure_ref(value.get()).get_instance()
        });
        let call_from_java_raw: *mut dyn Fn(InstanceWrapper) -> Instance =
            Box::into_raw(call_from_java);
        let call_from_java_as_i8_16 = unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) };
        kt_func1.internal_closure_raw = Some(call_from_java_as_i8_16);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(call_from_java_as_i8_16);
        let lumia_comparator = jvm
            .create_instance(
                "rt.lea.LumiaKtFunc1",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        kt_func1.instance = Some(lumia_comparator);
        Box::pin(kt_func1)
    }
    pub fn invoke(&self, val1: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance.as_ref().unwrap(), "invoke", &[val1])
            .unwrap();
        R::from_instance(result)
    }
}

pub(crate) struct KtFunc2<P1, P2, R, F>
    where
        P1: FromInstance,
        P2: FromInstance,
        F: Fn(P1, P2) -> R,
        R: GetEnvTrait + FromInstance,
{
    closure: F,
    instance: Option<Instance>,
    internal_closure_raw: Option<[i8; 16]>,
    _p1: PhantomData<P1>,
    _p2: PhantomData<P2>,
}

impl<P1, P2, R, F> GetEnvTrait for KtFunc2<P1, P2, R, F>
    where
        P1: FromInstance,
        P2: FromInstance,
        F: Fn(P1, P2) -> R,
        R: GetEnvTrait + FromInstance,
{
    fn get_instance(&self) -> Instance {
        Jvm::attach_thread()
            .unwrap()
            .clone_instance(self.instance.as_ref().unwrap())
            .unwrap()
    }
}

impl<P1, P2, R, F> KtFunc2<P1, P2, R, F>
    where
        P1: FromInstance,
        P2: FromInstance,
        F: Fn(P1, P2) -> R,
        R: GetEnvTrait + FromInstance,
{
    pub fn new(closure: F) -> Pin<Box<KtFunc2<P1, P2, R, F>>> {
        let jvm = Jvm::attach_thread().unwrap();
        let mut kt_func2: KtFunc2<P1, P2, R, F> = KtFunc2 {
            closure,
            instance: None,
            internal_closure_raw: None,
            _p1: Default::default(),
            _p2: Default::default(),
        };
        let closure_ref = &kt_func2.closure;
        let call_from_java = Box::new(|value: InstanceWrapper| -> Instance {
            let value = value.get::<KtPairWrapper<P1, P2>>();
            let (val1, val2) = value.get_pair();
            closure_ref(val1, val2).get_instance()
        });
        let call_from_java_raw: *mut dyn Fn(InstanceWrapper) -> Instance =
            Box::into_raw(call_from_java);
        let call_from_java_as_i8_16 = unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) };
        kt_func2.internal_closure_raw = Some(call_from_java_as_i8_16);
        let lumia_function = instance_from_i8_16::<"rt.lea.LumiaFunction">(call_from_java_as_i8_16);
        let lumia_comparator = jvm
            .create_instance(
                "rt.lea.LumiaKtFunc2",
                &[InvocationArg::try_from(lumia_function).unwrap()],
            )
            .unwrap();
        kt_func2.instance = Some(lumia_comparator);
        Box::pin(kt_func2)
    }
    pub fn invoke(&self, val1: InvocationArg, val2: InvocationArg) -> R {
        let jvm = Jvm::attach_thread().unwrap();
        let result = jvm
            .invoke(&self.instance.as_ref().unwrap(), "invoke", &[val1, val2])
            .unwrap();
        R::from_instance(result)
    }
}

impl FromInstance for () {
    fn from_instance(_instance: Instance) -> Self {
        ()
    }
}

impl GetEnvTrait for () {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke_static("javax.lang.model.util.Types", "getNullType", &[])
            .unwrap()
    }
}
