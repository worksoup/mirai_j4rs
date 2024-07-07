use crate::POINTER_SIZE;
use j4rs::errors::J4RsError;
use j4rs::prelude::*;
use j4rs_derive::*;
use mj_base::data_wrapper::DataWrapper;
use mj_base::env::TryFromInstanceTrait;
use std::intrinsics::transmute;

fn lumia_func_apply_internal<T>(
    raw_pointer_instance: Instance,
    arg: Instance,
) -> Result<T, String> {
    let func_raw: [i8; POINTER_SIZE] = Jvm::attach_thread()
        .unwrap()
        .to_rust(raw_pointer_instance)
        .unwrap();
    let func: *mut dyn Fn(DataWrapper<Instance>) -> Result<T, J4RsError> =
        unsafe { transmute(func_raw) };
    let val = DataWrapper::try_from_instance(arg)
        .and_then(|data| unsafe { (*func)(data) });
    val.map_err(|error| format!("{}", error))
}
#[call_from_java("rt.lea.function.LumiaConsumer.nativeAccept")]
fn lumia_consumer_accept(consumer_as_i8_16: Instance, arg: Instance) {
    let _ = lumia_func_apply_internal::<()>(consumer_as_i8_16, arg);
}

#[call_from_java("rt.lea.function.LumiaFunction.nativeApply")]
fn lumia_function_apply(
    function_raw_as_i8_16: Instance,
    arg: Instance,
) -> Result<Instance, String> {
    lumia_func_apply_internal::<Instance>(function_raw_as_i8_16, arg)
}
#[call_from_java("rt.lea.function.LumiaSupplier.nativeGet")]
fn lumia_supplier_get(raw_pointer_instance: Instance) -> Result<Instance, String> {
    let func_raw: [i8; POINTER_SIZE] = Jvm::attach_thread()
        .unwrap()
        .to_rust(raw_pointer_instance)
        .unwrap();
    let func: *mut dyn Fn() -> Result<Instance, J4RsError> = unsafe { transmute(func_raw) };
    unsafe { (*func)() }.map_err(|error| format!("{}", error))
}

// #[call_from_java("rt.lea.LumiaKtFunc0.nativeInvoke")]
// fn lumia_kt_func_0_invoke(kt_func_0_raw_as_i8_16: Instance) -> Result<Instance, String> {
//     let kt_func_0_raw: [i8; 16] = Jvm::attach_thread()
//         .unwrap()
//         .to_rust(kt_func_0_raw_as_i8_16)
//         .unwrap();
//     let kt_func_0: *mut dyn Fn() -> Instance = unsafe { transmute(kt_func_0_raw) };
//     let value = unsafe { (*kt_func_0)() };
//     Ok(value)
// }
// #[call_from_java("rt.lea.function.LumiaPredicate.nativeTest")]
// fn lumia_predicate_test(
//     predicate_raw_as_i8_16: Instance,
//     item: Instance,
// ) -> Result<Instance, String> {
//     let predicate_raw: [i8; 16] = Jvm::attach_thread()
//         .unwrap()
//         .to_rust(predicate_raw_as_i8_16)
//         .unwrap();
//     let function: *mut dyn Fn(DataWrapper<Instance>) -> Result<InvocationArg, J4RsError> =
//         unsafe { transmute(predicate_raw) };
//     let value = unsafe { (*function)(DataWrapper::from_instance(item)) }
//         .map_err(|error| format!("{}", error))?;
//     Instance::try_from(value).map_err(|error| format!("{}", error))
// }
