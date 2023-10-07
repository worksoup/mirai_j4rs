use j4rs::{Instance, InvocationArg, Jvm};
use crate::contact::bot::Bot;
use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::ffi::{KtFunc1, KtFunc2};

pub struct QRCodeLoginListener {}

impl FromInstance for QRCodeLoginListener {
    fn from_instance(instance: Instance) -> Self {
        todo!()
    }
}

impl GetEnvTrait for QRCodeLoginListener {
    fn get_instance(&self) -> Instance {
        todo!()
    }
}

pub struct DeviceVerificationRequests {}

impl FromInstance for DeviceVerificationRequests {
    fn from_instance(instance: Instance) -> Self {
        todo!()
    }
}

pub struct DeviceVerificationResult {}

impl FromInstance for DeviceVerificationResult {
    fn from_instance(instance: Instance) -> Self {
        todo!()
    }
}

impl GetEnvTrait for DeviceVerificationResult {
    fn get_instance(&self) -> Instance {
        todo!()
    }
}


pub trait LoginSolverTrait<const IS_SLIDER_CAPTCHA_SUPPORTED: bool> {
    fn on_solve_slider_captcha(bot: Bot, url: String) -> String;
    fn on_solve_pic_captcha(bot: Bot, url: Vec<i8>) -> String;
    fn create_qrcode_login_listener(bot: Bot) -> QRCodeLoginListener;
    fn on_solve_device_verification(bot: Bot, requests: DeviceVerificationRequests) -> DeviceVerificationResult;
    fn __instance() -> Instance {
        impl FromInstance for String {
            fn from_instance(instance: Instance) -> Self {
                let jvm = Jvm::attach_thread().unwrap();
                jvm.to_rust(instance).unwrap()
            }
        }
        impl GetEnvTrait for String {
            fn get_instance(&self) -> Instance {
                Instance::try_from(InvocationArg::try_from(self).unwrap()).unwrap()
            }
        }
        impl FromInstance for Vec<i8> {
            fn from_instance(instance: Instance) -> Self {
                let jvm = Jvm::attach_thread().unwrap();
                jvm.to_rust(instance).unwrap()
            }
        }
        let on_solve_slider_captcha = KtFunc2::new(Self::on_solve_slider_captcha).get_instance();
        let on_solve_pic_captcha = KtFunc2::new(Self::on_solve_pic_captcha).get_instance();
        let create_qrcode_login_listener = KtFunc1::new(Self::create_qrcode_login_listener).get_instance();
        let on_solve_device_verification = KtFunc2::new(Self::on_solve_device_verification).get_instance();
        let is_slider_captcha_supported = InvocationArg::try_from(IS_SLIDER_CAPTCHA_SUPPORTED).unwrap();
        let on_solve_slider_captcha = InvocationArg::try_from(on_solve_slider_captcha).unwrap();
        let on_solve_pic_captcha = InvocationArg::try_from(on_solve_pic_captcha).unwrap();
        let create_qrcode_login_listener = InvocationArg::try_from(create_qrcode_login_listener).unwrap();
        let on_solve_device_verification = InvocationArg::try_from(on_solve_device_verification).unwrap();
        let jvm = Jvm::attach_thread().unwrap();
        jvm.create_instance("rt.lea.LumiaLoginSolver", &[
            on_solve_slider_captcha, on_solve_pic_captcha, is_slider_captcha_supported, create_qrcode_login_listener, on_solve_device_verification
        ]).unwrap()
    }
}
