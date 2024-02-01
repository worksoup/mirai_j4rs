#[cfg(test)]
mod tests {
    use crate::{
        comparator::Comparator, consumer::Consumer, function::Function, kt_func_0::KtFunc0,
        kt_func_1::KtFunc1, kt_func_2::KtFunc2, predicate::Predicate,
    };
    use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
    use mj_base::env::{FromInstance, GetEnvTrait};
    use std::cmp::Ordering;

    struct X {
        instance: Instance,
    }

    impl GetEnvTrait for X {
        fn get_instance(&self) -> Instance {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.clone_instance(&self.instance).unwrap()
        }
    }

    impl X {
        fn fuck(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            jvm.chain(&self.instance)
                .unwrap()
                .invoke("getClass", &[])
                .unwrap()
                .invoke("toString", &[])
                .unwrap()
                .to_rust()
                .unwrap()
        }
    }

    impl FromInstance for X {
        fn from_instance(instance: Instance) -> Self {
            X { instance }
        }
    }

    fn get_a_jvm_for_test() -> Jvm {
        JvmBuilder::new()
            .classpath_entry(ClasspathEntry::new(
                "/home/leart/Applications/dev/Mirai/MiraiRS/MiraiRS.jar",
            ))
            .build()
            .unwrap_or_else(|_| Jvm::attach_thread().unwrap())
    }

    #[test]
    fn closure_to_consumer_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = |x: X| {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
        };
        let consumer = Consumer::new(&f);
        let test_instance = InvocationArg::try_from(true).unwrap();
        consumer.accept(test_instance);
        let _ = consumer.drop_and_to_raw();
    }

    #[test]
    fn closure_to_function_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = |x: X| -> X {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
            x
        };
        let function = Function::new(&f);
        let test_instance = InvocationArg::try_from(true).unwrap();
        let x = function.apply(test_instance);
        let _ = function.drop_and_to_raw();
        println!("a = {a}\nThe class name is `{}`.", x.fuck());
    }

    #[test]
    fn closure_to_comparator_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x1: X, x2: X| -> Ordering {
            let jvm = Jvm::attach_thread().unwrap(); // jvm 不能直接捕获，否则会卡死或崩溃。
            let x1 = x1.get_instance();
            let x2 = x2.get_instance();
            let val1: i32 = jvm.to_rust(x1).unwrap();
            let val2: i32 = jvm.to_rust(x2).unwrap();
            val1.cmp(&val2)
        };
        let comparator = Comparator::new(&f);
        let test_instance1 = InvocationArg::try_from(22).unwrap_or_else(|err| panic!("{}", err));
        let test_instance2 = InvocationArg::try_from(55).unwrap();
        let x = comparator.compare(test_instance1, test_instance2);
        let _ = comparator.drop_and_to_raw();
        println!("a = {a}\nThe ordering is `{:?}`.", x);
    }

    #[test]
    fn closure_to_predicate_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x1: X| -> bool {
            let jvm = Jvm::attach_thread().unwrap(); // jvm不能直接捕获，否则会卡死。
            let val1: i32 = jvm.to_rust(x1.get_instance()).unwrap();
            val1 > 0
        };
        let predicate = Predicate::new(&f);
        // println!("sleep");
        // sleep(std::time::Duration::from_millis(10000));
        let test_value = InvocationArg::try_from(22).unwrap_or_else(|err| panic!("{}", err));
        let x = predicate.test(test_value);
        let _ = predicate.drop_and_to_raw();
        println!("a = {a}\n And `test_value > 0` is `{:?}`.", x);
    }

    #[test]
    fn closure_to_kt_func_0_works() {
        let jvm = get_a_jvm_for_test();
        let a = 2;
        let f = || -> X {
            let _jvm = Jvm::attach_thread().unwrap(); // jvm不能直接捕获，否则会卡死。
            let b = InvocationArg::try_from(true)
                .unwrap()
                .into_primitive()
                .unwrap();
            let instance = _jvm.create_instance("java.lang.Boolean", &[b]).unwrap(); // 需要通过参数对象创建对象，不能直接 Instance::try_from, 否则会出错。
            X { instance }
        };
        let kt_func_0 = KtFunc0::new(&f);
        let x = kt_func_0.invoke();
        let _ = kt_func_0.drop_and_to_raw();
        println!(
            "a = {a}\n And `x` is `{}`.",
            jvm.to_rust::<bool>(jvm.cast(&x.get_instance(), "java.lang.Boolean").unwrap())
                .unwrap()
        );
    }

    #[test]
    fn closure_to_kt_func_1_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let f = |x: X| -> X {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
            x
        };
        let kt_func_1 = KtFunc1::new(&f);
        let test_instance = InvocationArg::try_from(true).unwrap();
        let x = kt_func_1.invoke(test_instance);
        let _ = kt_func_1.drop_and_to_raw();
        println!("a = {a}\nThe class name is `{}`.", x.fuck());
    }

    #[test]
    fn closure_to_kt_func_2_works() {
        let top_jvm = get_a_jvm_for_test();
        let a = 2;
        let f = move |x1: X, x2: X| -> X {
            let jvm = Jvm::attach_thread().unwrap(); // jvm 不能直接捕获，否则会卡死或崩溃。
            let x1 = x1.get_instance();
            let x2 = x2.get_instance();
            let val1: i32 = jvm.to_rust(x1).unwrap();
            let val2: i32 = jvm.to_rust(x2).unwrap();
            let b = InvocationArg::try_from(val1 - val2)
                .unwrap()
                .into_primitive()
                .unwrap();
            let instance = jvm.create_instance("java.lang.Integer", &[b]).unwrap(); // 需要通过参数对象创建对象，不能直接 Instance::try_from, 否则会出错。
            X { instance }
        };
        let kt_func_2 = KtFunc2::new(&f);
        let test_instance1 = InvocationArg::try_from(22).unwrap_or_else(|err| panic!("{}", err));
        let test_instance2 = InvocationArg::try_from(55).unwrap();
        let x = kt_func_2.invoke(test_instance1, test_instance2);
        let _ = kt_func_2.drop_and_to_raw();
        println!(
            "a = {a}\nThe ordering is `{:?}`.",
            top_jvm.to_rust::<i32>(x.get_instance()).unwrap()
        );
    }
}
