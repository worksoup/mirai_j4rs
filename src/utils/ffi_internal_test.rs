#[cfg(test)]
mod tests {
    use crate::{
        env::{FromInstance, GetEnvTrait},
        utils::ffi::{Comparator, Consumer, Function, Predicate},
    };
    use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
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
        match JvmBuilder::new()
            .classpath_entry(ClasspathEntry::new(
                "/run/media/leart/5A98CD5F98CD3A71/Users/15102/Works/Mirai/MiraiRS/jvm_side.jar",
            ))
            .build()
        {
            Ok(jvm) => jvm,
            Err(_) => Jvm::attach_thread().unwrap(),
        }
    }

    #[test]
    fn closure_to_consumer_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let consumer = Consumer::new(|x: X| {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
        });
        let test_instance = InvocationArg::try_from(true).unwrap();
        consumer.accept(test_instance);
    }

    #[test]
    fn closure_to_function_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let function = Function::new(|x: X| -> X {
            println!("a = {a}\nThe class name is `{}`.", x.fuck());
            x
        });
        let test_instance = InvocationArg::try_from(true).unwrap();
        let x = function.apply(test_instance);
        println!("a = {a}\nThe class name is `{}`.", x.fuck());
    }

    #[test]
    fn closure_to_comparator_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let comparator = Comparator::new(move |x1: &X, x2: &X| -> Ordering {
            let jvm = Jvm::attach_thread().unwrap(); // jvm 不能直接捕获，否则会卡死或崩溃。
            let x1 = x1.get_instance();
            let x2 = x2.get_instance();
            let val1: i32 = jvm.to_rust(x1).unwrap();
            let val2: i32 = jvm.to_rust(x2).unwrap();
            val1.cmp(&val2)
        });
        let test_instance1 = InvocationArg::try_from(22).unwrap_or_else(|err| panic!("{}", err));
        let test_instance2 = InvocationArg::try_from(55).unwrap();
        let x = comparator.compare(test_instance1, test_instance2);
        println!("a = {a}\nThe ordering is `{:?}`.", x);
    }

    #[test]
    fn closure_to_predicate_works() {
        let _jvm = get_a_jvm_for_test();
        let a = 2;
        let predicate = Predicate::new(move |x1: X| -> bool {
            let jvm = Jvm::attach_thread().unwrap(); // jvm不能直接捕获，否则会卡死。
            let val1: i32 = jvm.to_rust(x1.get_instance()).unwrap();
            val1 > 0
        });
        // println!("sleep");
        // sleep(std::time::Duration::from_millis(10000));
        let test_value = InvocationArg::try_from(22).unwrap_or_else(|err| panic!("{}", err));
        let x = predicate.test(test_value);
        println!("a = {a}\n And `test_value > 0` is `{:?}`.", x);
    }
}
