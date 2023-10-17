use crate::contact::bot::Bot;
use crate::env::{FromInstance, GetEnvTrait};
use crate::utils::ffi::callable_objects_in_jvm::{
    kt_func_0::KtFunc0, kt_func_1::KtFunc1, kt_func_2::KtFunc2,
};
use crate::utils::internal::data_wrapper::DataWrapper;
use contact_derive::GetInstanceDerive;
use j4rs::{Instance, InvocationArg, Jvm};
use std::ops::Deref;
use crate::utils::ffi::callable_objects_in_jvm::kt_func_0::KtFunc0Raw;
use crate::utils::ffi::callable_objects_in_jvm::kt_func_1::KtFunc1Raw;
use crate::utils::ffi::callable_objects_in_jvm::kt_func_2::KtFunc2Raw;

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

impl GetEnvTrait for State {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.static_class_field("net.mamoe.mirai.auth.QRCodeLoginListener$State", match self {
            State::WaitingForScan => { "WAITING_FOR_SCAN" }
            State::WaitingForConfirm => { "WAITING_FOR_CONFIRM" }
            State::Cancelled => { "CANCELLED" }
            State::Timeout => { "TIMEOUT" }
            State::Confirmed => { "CONFIRMED" }
            State::Default => { "DEFAULT" }
        }).unwrap()
    }
}

pub struct QrCodeLoginListener {
    instance: Option<Instance>,
    on_fetch_qrcode: Option<Box<dyn Fn(Bot, DataWrapper<Vec<i8>>) -> DataWrapper<()>>>,
    on_state_changed: Option<Box<dyn Fn(Bot, State) -> DataWrapper<()>>>,
    on_interval_loop: Option<Box<dyn Fn() -> DataWrapper<()>>>,
    on_completed: Option<Box<dyn Fn() -> DataWrapper<()>>>,
    _1: Option<KtFunc2Raw>,
    _2: Option<KtFunc2Raw>,
    _3: Option<KtFunc0Raw>,
    _4: Option<KtFunc0Raw>,
}

impl QrCodeLoginListener {
    pub fn new<
        const QR_CODE_SIZE: i32,
        const QR_CODE_MARGIN: i32,
        const QR_CODE_EC_LEVEL: i32,
        const QR_CODE_STATE_UPDATE_INTERVAL: i64,
        F: Fn(Bot, &Vec<i8>),
        S: Fn(Bot, State),
        I: Fn(),
        C: Fn(),
    >(
        on_fetch_qrcode: &'static F,
        on_state_changed: &'static S,
        on_interval_loop: &'static I,
        on_completed: &'static C,
    ) -> Self {
        let mut r = Self {
            instance: None,
            on_fetch_qrcode: Some(Box::new(|bot: Bot, data: DataWrapper<Vec<i8>>| -> DataWrapper<()> {
                on_fetch_qrcode(bot, data.deref()).into()
            })),
            on_state_changed: Some(Box::new(|bot: Bot, state: State| -> DataWrapper<()> {
                on_state_changed(bot, state).into()
            })),
            on_interval_loop: Some(Box::new(|| -> DataWrapper<()> {
                on_interval_loop().into()
            })),
            on_completed: Some(Box::new(|| -> DataWrapper<()> {
                on_completed().into()
            })),
            _1: None,
            _2: None,
            _3: None,
            _4: None,
        };
        let jvm = Jvm::attach_thread().unwrap();

        let qr_code_size = InvocationArg::try_from(QR_CODE_SIZE)
            .unwrap()
            .into_primitive()
            .unwrap();
        let qr_code_margin = InvocationArg::try_from(QR_CODE_MARGIN)
            .unwrap()
            .into_primitive()
            .unwrap();
        let qr_code_ec_level = InvocationArg::try_from(QR_CODE_EC_LEVEL)
            .unwrap()
            .into_primitive()
            .unwrap();
        let qr_code_state_update_interval =
            InvocationArg::try_from(QR_CODE_STATE_UPDATE_INTERVAL)
                .unwrap()
                .into_primitive()
                .unwrap();

        r._1 = Some(KtFunc2::new(r.on_fetch_qrcode.as_ref().unwrap()).drop_and_to_raw());
        r._2 = Some(KtFunc2::new(r.on_state_changed.as_ref().unwrap()).drop_and_to_raw());
        r._3 = Some(KtFunc0::new(r.on_interval_loop.as_ref().unwrap()).drop_and_to_raw());
        r._4 = Some(KtFunc0::new(r.on_completed.as_ref().unwrap()).drop_and_to_raw());

        let on_fetch_qrcode = InvocationArg::try_from(r._1.as_ref().unwrap().get_instance()).unwrap();
        let on_state_changed = InvocationArg::try_from(r._2.as_ref().unwrap().get_instance()).unwrap();
        let on_interval_loop = InvocationArg::try_from(r._3.as_ref().unwrap().get_instance()).unwrap();
        let on_completed = InvocationArg::try_from(r._4.as_ref().unwrap().get_instance()).unwrap();

        let instance = jvm
            .create_instance(
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
            .unwrap();
        r.instance = Some(instance);
        r
    }

    pub fn on_fetch_qrcode(&self, bot: Bot, data: &Vec<i8>) {
        if let Some(ref internal_on_fetch_qrcode) = self.on_fetch_qrcode {
            internal_on_fetch_qrcode(bot, data.clone().into());
        } else {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = self.instance.as_ref().unwrap();
            let bot = InvocationArg::try_from(bot.get_instance()).unwrap();
            let data = {
                let mut tmp = Vec::new();
                for item in data {
                    tmp.push(
                        InvocationArg::try_from(item)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    );
                }
                tmp
            };
            let data = jvm.create_java_array("byte", &data).unwrap();
            let data = InvocationArg::try_from(data).unwrap().into_primitive().unwrap();
            jvm.invoke(instance, "onFetchQRCode", &[bot, data]).unwrap();
        }
    }
    pub fn on_state_changed(&self, bot: Bot, state: State) {
        if let Some(ref internal_on_state_changed) = self.on_state_changed {
            internal_on_state_changed(bot, state);
        } else {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = self.instance.as_ref().unwrap();
            let bot = InvocationArg::try_from(bot.get_instance()).unwrap();
            let state = InvocationArg::try_from(state.get_instance()).unwrap();
            jvm.invoke(instance, "onStateChanged", &[bot, state]).unwrap();
        }
    }
    pub fn on_interval_loop(&self) {
        if let Some(ref internal_on_interval_loop) = self.on_interval_loop {
            internal_on_interval_loop();
        } else {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = self.instance.as_ref().unwrap();
            jvm.invoke(instance, "onIntervalLoop", &[]).unwrap();
        }
    }
    pub fn on_completed(&self) {
        if let Some(ref internal_on_completed) = self.on_completed {
            internal_on_completed();
        } else {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = self.instance.as_ref().unwrap();
            jvm.invoke(instance, "onCompleted", &[]).unwrap();
        }
    }
}

impl GetEnvTrait for QrCodeLoginListener {
    fn get_instance(&self) -> Instance {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.clone_instance(self.instance.as_ref().unwrap()).unwrap()
    }
}

impl FromInstance for QrCodeLoginListener {
    fn from_instance(instance: Instance) -> Self {
        Self {
            instance: Some(instance),
            on_fetch_qrcode: None,
            on_state_changed: None,
            on_interval_loop: None,
            on_completed: None,
            _1: None,
            _2: None,
            _3: None,
            _4: None,
        }
    }
}

pub trait QrCodeLoginListenerTrait
    where
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

pub trait LoginSolverTrait
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
    fn __create_qrcode_login_listener(bot: Bot) -> QrCodeLoginListener {
        Self::create_qrcode_login_listener(bot).into()
    }
    fn __instance() -> (Instance, KtFunc2Raw, KtFunc2Raw, KtFunc1Raw, KtFunc2Raw) {
        let jvm = Jvm::attach_thread().unwrap();

        let _1 = KtFunc2::new(&Self::__on_solve_slider_captcha);
        let _2 = KtFunc2::new(&Self::__on_solve_pic_captcha);
        let _3 = KtFunc1::new(&Self::__create_qrcode_login_listener);
        let _4 = KtFunc2::new(&Self::on_solve_device_verification);

        let on_solve_slider_captcha = InvocationArg::try_from(_1.to_instance()).unwrap();
        let on_solve_pic_captcha = InvocationArg::try_from(_2.to_instance()).unwrap();
        let is_slider_captcha_supported =
            InvocationArg::try_from(Self::IS_SLIDER_CAPTCHA_SUPPORTED).unwrap();
        let create_qrcode_login_listener =
            InvocationArg::try_from(_3.to_instance()).unwrap();
        let on_solve_device_verification =
            InvocationArg::try_from(_4.to_instance()).unwrap();
        (jvm.create_instance(
            "rt.lea.LumiaLoginSolver",
            &[
                on_solve_slider_captcha,
                on_solve_pic_captcha,
                is_slider_captcha_supported,
                create_qrcode_login_listener,
                on_solve_device_verification,
            ],
        )
             .unwrap(), _1.drop_and_to_raw(), _2.drop_and_to_raw(), _3.drop_and_to_raw(), _4.drop_and_to_raw())
    }
}
