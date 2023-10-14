use crate::contact::bot::Bot;
use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::ffi::callable_objects_in_jvm::{
    kt_func_0::KtFunc0, kt_func_1::KtFunc1, kt_func_2::KtFunc2,
};
use crate::utils::internal::data_wrapper::DataWrapper;
use contact_derive::GetInstanceDerive;
use j4rs::{Instance, InvocationArg, Jvm};
use std::ops::Deref;

pub enum State {
    /// 等待扫描中，请在此阶段请扫描二维码.
    WaitingForScan,
    /// 二维码已扫描，等待扫描端确认登录.
    WaitingForConfirm,
    /// 扫描后取消了确认.
    Cancelled,
    /// 二维码超时，必须重新获取二维码.
    Timeout,
    /// 二维码已确认，将会继续登录.
    Confirmed,
    /// 默认状态，在登录前通常为此状态.
    Default,
}

impl FromInstance for State {
    fn from_instance(instance: Instance) -> Self {
        let mp: String = Jvm::attach_thread().unwrap().to_rust(instance).unwrap();
        match mp.as_str() {
            "WAITING_FOR_SCAN" => State::WaitingForScan,
            "WAITING_FOR_CONFIRM" => State::WaitingForConfirm,
            "CANCELLED" => State::Cancelled,
            "TIMEOUT" => State::Timeout,
            "CONFIRMED" => State::Confirmed,
            _ => State::Default,
        }
    }
}

pub trait QrCodeLoginListenerTrait where
    Self: FromInstance + GetEnvTrait,
{
    const QR_CODE_SIZE: i32 = 3;
    const QR_CODE_MARGIN: i32 = 4;
    const QR_CODE_EC_LEVEL: i32 = 2;
    const QR_CODE_STATE_UPDATE_INTERVAL: i64 = 5000;
    fn on_fetch_qrcode(bot: Bot, data: &Vec<i8>);
    fn on_state_changed(bot: Bot, state: State);
    fn on_interval_loop();
    fn on_completed();
}

trait InternalQrCodeLoginListenerTrait: QrCodeLoginListenerTrait + GetEnvTrait {
    fn __on_fetch_qrcode(bot: Bot, data: DataWrapper<Vec<i8>>) -> DataWrapper<()> {
        Self::on_fetch_qrcode(bot, data.deref()).into()
    }

    fn __on_state_changed(bot: Bot, state: State) -> DataWrapper<()> {
        Self::on_state_changed(bot, state).into()
    }

    fn __on_interval_loop() -> DataWrapper<()> {
        Self::on_interval_loop().into()
    }
    fn __on_completed() -> DataWrapper<()> {
        Self::on_completed().into()
    }
    fn __instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        let qr_code_size = InvocationArg::try_from(Self::QR_CODE_SIZE)
            .unwrap()
            .into_primitive()
            .unwrap();
        let qr_code_margin = InvocationArg::try_from(Self::QR_CODE_MARGIN)
            .unwrap()
            .into_primitive()
            .unwrap();
        let qr_code_ec_level = InvocationArg::try_from(Self::QR_CODE_EC_LEVEL)
            .unwrap()
            .into_primitive()
            .unwrap();
        let qr_code_state_update_interval = InvocationArg::try_from(Self::QR_CODE_STATE_UPDATE_INTERVAL)
            .unwrap()
            .into_primitive()
            .unwrap();

        let on_fetch_qrcode = InvocationArg::try_from(KtFunc2::new(&Self::__on_fetch_qrcode).to_instance()).unwrap();
        let on_state_changed = InvocationArg::try_from(KtFunc2::new(&Self::__on_state_changed).to_instance()).unwrap();
        let on_interval_loop = InvocationArg::try_from(KtFunc0::new(&Self::__on_interval_loop).to_instance()).unwrap();
        let on_completed = InvocationArg::try_from(KtFunc0::new(&Self::__on_completed).to_instance()).unwrap();

        jvm.create_instance(
            "rt.lea.LumiaQrCodeLoginListener",
            &[
                qr_code_size,
                qr_code_margin,
                qr_code_ec_level,
                qr_code_state_update_interval,
                on_fetch_qrcode,
                on_state_changed,
                on_interval_loop,
                on_completed,
            ],
        )
            .unwrap()
    }
}

impl<T: QrCodeLoginListenerTrait> InternalQrCodeLoginListenerTrait for T {}

#[derive(GetInstanceDerive)]
pub struct SmsRequests {
    instance: Instance,
}

impl SmsRequests {
    pub fn get_country_code(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getUrl", &[]).unwrap())
            .unwrap()
    }
    pub fn get_phone_number(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getUrl", &[]).unwrap())
            .unwrap()
    }
    pub fn request_sms(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        let r = InvocationArg::try_from(self.get_instance()).unwrap();
        let _ = jvm
            .invoke_static("rt.lea.LumiaLoginSolverKt", "requestSmsOf", &[r])
            .unwrap();
    }
    pub fn solved(&self, code: &str) -> DeviceVerificationResult {
        let jvm = Jvm::attach_thread().unwrap();
        let code = InvocationArg::try_from(code).unwrap();
        let instance = jvm.invoke(&self.instance, "solved", &[code]).unwrap();
        DeviceVerificationResult { instance }
    }
}

pub struct FallbackRequests {
    instance: Instance,
}

impl FallbackRequests {
    pub fn get_url(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getUrl", &[]).unwrap())
            .unwrap()
    }
    pub fn solved(&self) -> DeviceVerificationResult {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "solved", &[]).unwrap();
        DeviceVerificationResult { instance }
    }
}

pub struct DeviceVerificationRequests {
    instance: Instance,
}

impl DeviceVerificationRequests {
    pub fn get_fallback(&self) -> FallbackRequests {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getFallback", &[]).unwrap();
        FallbackRequests { instance }
    }
    pub fn get_prefer_sms(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(jvm.invoke(&self.instance, "getPreferSms", &[]).unwrap())
            .unwrap()
    }
    pub fn get_sms(&self) -> SmsRequests {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm.invoke(&self.instance, "getSms", &[]).unwrap();
        SmsRequests { instance }
    }
}

impl FromInstance for DeviceVerificationRequests {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

#[derive(GetInstanceDerive)]
pub struct DeviceVerificationResult {
    instance: Instance,
}

impl FromInstance for DeviceVerificationResult {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }
}

pub trait LoginSolverTrait<
    QrCodeLoginListener,
> where
    QrCodeLoginListener: InternalQrCodeLoginListenerTrait,
{
    const IS_SLIDER_CAPTCHA_SUPPORTED: bool = true;
    fn on_solve_slider_captcha(bot: Bot, url: &str) -> String;
    fn on_solve_pic_captcha(bot: Bot, data: &Vec<i8>) -> String;
    fn create_qrcode_login_listener(bot: Bot) -> QrCodeLoginListener;
    fn on_solve_device_verification(
        bot: Bot,
        requests: DeviceVerificationRequests,
    ) -> DeviceVerificationResult;
    fn __on_solve_slider_captcha(bot: Bot, url: DataWrapper<String>) -> DataWrapper<String> {
        Self::on_solve_slider_captcha(bot, url.as_str()).into()
    }
    fn __on_solve_pic_captcha(bot: Bot, data: DataWrapper<Vec<i8>>) -> DataWrapper<String> {
        Self::on_solve_pic_captcha(bot, data.deref()).into()
    }
    fn __create_qrcode_login_listener(bot: Bot) -> DataWrapper<QrCodeLoginListener> {
        Self::create_qrcode_login_listener(bot).into()
    }
    fn __instance() -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        let on_solve_slider_captcha = InvocationArg::try_from(KtFunc2::new(&Self::__on_solve_slider_captcha).to_instance()).unwrap();
        let on_solve_pic_captcha = InvocationArg::try_from(KtFunc2::new(&Self::__on_solve_pic_captcha).to_instance()).unwrap();
        let is_slider_captcha_supported =
            InvocationArg::try_from(Self::IS_SLIDER_CAPTCHA_SUPPORTED).unwrap();
        let create_qrcode_login_listener =
            InvocationArg::try_from(KtFunc1::new(&Self::__create_qrcode_login_listener).to_instance()).unwrap();
        let on_solve_device_verification =
            InvocationArg::try_from(KtFunc2::new(&Self::on_solve_device_verification).to_instance()).unwrap();
        jvm.create_instance(
            "rt.lea.LumiaLoginSolver",
            &[
                on_solve_slider_captcha,
                on_solve_pic_captcha,
                is_slider_captcha_supported,
                create_qrcode_login_listener,
                on_solve_device_verification,
            ],
        )
            .unwrap()
    }
}
