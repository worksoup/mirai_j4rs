use std::ops::Deref;

use j4rs::errors::J4RsError;
use j4rs::{Instance, InvocationArg, Jvm};
use jbuchong::{
    java_type, AsInstanceDerive, AsInstanceTrait, Func1, GetInstanceDerive, JavaBytes, JavaString,
    KotlinUnit, TryFromInstanceDerive,
};
use jbuchong::{
    Func0, {GetInstanceTrait, TryFromInstanceTrait},
};
use jbuchong::{Func2, GetClassTypeTrait};

use crate::contact::Bot;

/// 该结构体未实现 [`LoginSolverTrait`], 如需使用相关功能请调用本实例的 `get_instance` 方法，获得 `Instance` 后直接操作。
#[derive(AsInstanceDerive, GetInstanceDerive, TryFromInstanceDerive)]
pub struct LoginSolver {
    instance: Instance,
}

#[java_type("net.mamoe.mirai.auth.QRCodeLoginListener$State")]
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

impl TryFromInstanceTrait for State {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        let mp: String = Jvm::attach_thread().unwrap().to_rust(instance).unwrap();
        Ok(match mp.as_str() {
            "WAITING_FOR_SCAN" => State::WaitingForScan,
            "WAITING_FOR_CONFIRM" => State::WaitingForConfirm,
            "CANCELLED" => State::Cancelled,
            "TIMEOUT" => State::Timeout,
            "CONFIRMED" => State::Confirmed,
            _ => State::Default,
        })
    }
}

impl GetInstanceTrait for State {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        Ok(jvm
            .static_class_field(
                <Self as GetClassTypeTrait>::get_type_name(),
                match self {
                    State::WaitingForScan => "WAITING_FOR_SCAN",
                    State::WaitingForConfirm => "WAITING_FOR_CONFIRM",
                    State::Cancelled => "CANCELLED",
                    State::Timeout => "TIMEOUT",
                    State::Confirmed => "CONFIRMED",
                    State::Default => "DEFAULT",
                },
            )
            .unwrap())
    }
}

pub struct QrCodeLoginListener {
    instance: Option<Instance>,
    on_fetch_qrcode: Option<Func2<Bot, Vec<i8>, KotlinUnit>>,
    on_state_changed: Option<Func2<Bot, State, KotlinUnit>>,
    on_interval_loop: Option<Func0<KotlinUnit>>,
    on_completed: Option<Func0<KotlinUnit>>,
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
        _on_fetch_qrcode: &'static F,
        _on_state_changed: &'static S,
        _on_interval_loop: &'static I,
        _on_completed: &'static C,
    ) -> Self {
        // let mut r = Self {
        //     instance: None,
        //     on_fetch_qrcode: Some(Box::new(
        //         |bot: Bot, data: DataWrapper<Vec<i8>>| -> DataWrapper<()> {
        //             on_fetch_qrcode(bot, data.deref()).into()
        //         },
        //     )),
        //     on_state_changed: Some(Box::new(|bot: Bot, state: State| -> DataWrapper<()> {
        //         on_state_changed(bot, state).into()
        //     })),
        //     on_interval_loop: Some(Box::new(|| -> DataWrapper<()> {
        //         on_interval_loop().into()
        //     })),
        //     on_completed: Some(Box::new(|| -> DataWrapper<()> { on_completed().into() })),
        //     _1: None,
        //     _2: None,
        //     _3: None,
        //     _4: None,
        // };
        // let jvm = Jvm::attach_thread().unwrap();
        //
        // let qr_code_size = InvocationArg::try_from(QR_CODE_SIZE)
        //     .unwrap()
        //     .into_primitive()
        //     .unwrap();
        // let qr_code_margin = InvocationArg::try_from(QR_CODE_MARGIN)
        //     .unwrap()
        //     .into_primitive()
        //     .unwrap();
        // let qr_code_ec_level = InvocationArg::try_from(QR_CODE_EC_LEVEL)
        //     .unwrap()
        //     .into_primitive()
        //     .unwrap();
        // let qr_code_state_update_interval = InvocationArg::try_from(QR_CODE_STATE_UPDATE_INTERVAL)
        //     .unwrap()
        //     .into_primitive()
        //     .unwrap();
        //
        // r._1 = Some(KtFunc2::new(r.on_fetch_qrcode.as_ref().unwrap()).drop_and_to_raw());
        // r._2 = Some(KtFunc2::new(r.on_state_changed.as_ref().unwrap()).drop_and_to_raw());
        // r._3 = Some(KtFunc0::new(r.on_interval_loop.as_ref().unwrap()).drop_and_to_raw());
        // r._4 = Some(KtFunc0::new(r.on_completed.as_ref().unwrap()).drop_and_to_raw());
        //
        // let on_fetch_qrcode =
        //     InvocationArg::try_from(r._1.as_ref().unwrap().get_instance()).unwrap();
        // let on_state_changed =
        //     InvocationArg::try_from(r._2.as_ref().unwrap().get_instance()).unwrap();
        // let on_interval_loop =
        //     InvocationArg::try_from(r._3.as_ref().unwrap().get_instance()).unwrap();
        // let on_completed = InvocationArg::try_from(r._4.as_ref().unwrap().get_instance()).unwrap();
        //
        // let instance = jvm
        //     .create_instance(
        //         "rt.lea.LumiaQrCodeLoginListener",
        //         &[
        //             qr_code_size,
        //             qr_code_margin,
        //             qr_code_ec_level,
        //             qr_code_state_update_interval,
        //             on_fetch_qrcode,
        //             on_state_changed,
        //             on_interval_loop,
        //             on_completed,
        //         ],
        //     )
        //     .unwrap();
        // r.instance = Some(instance);
        // r
        todo!()
    }

    pub fn on_fetch_qrcode(&self, bot: Bot, data: &Vec<i8>) {
        if let Some(ref internal_on_fetch_qrcode) = self.on_fetch_qrcode {
            internal_on_fetch_qrcode.call(bot, data.clone());
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
            let data = InvocationArg::try_from(data)
                .unwrap()
                .into_primitive()
                .unwrap();
            jvm.invoke(instance, "onFetchQRCode", &[bot, data]).unwrap();
        }
    }
    pub fn on_state_changed(&self, bot: Bot, state: State) {
        if let Some(ref internal_on_state_changed) = self.on_state_changed {
            internal_on_state_changed.call(bot, state);
        } else {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = self.instance.as_ref().unwrap();
            let bot = InvocationArg::try_from(bot.get_instance()).unwrap();
            let state = InvocationArg::try_from(state.get_instance()).unwrap();
            jvm.invoke(instance, "onStateChanged", &[bot, state])
                .unwrap();
        }
    }
    pub fn on_interval_loop(&self) {
        if let Some(ref internal_on_interval_loop) = self.on_interval_loop {
            internal_on_interval_loop.call();
        } else {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = self.instance.as_ref().unwrap();
            jvm.invoke(instance, "onIntervalLoop", InvocationArg::empty())
                .unwrap();
        }
    }
    pub fn on_completed(&self) {
        if let Some(ref internal_on_completed) = self.on_completed {
            internal_on_completed.call();
        } else {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = self.instance.as_ref().unwrap();
            jvm.invoke(instance, "onCompleted", InvocationArg::empty())
                .unwrap();
        }
    }
}

impl GetInstanceTrait for QrCodeLoginListener {
    fn get_instance(&self) -> Result<Instance, J4RsError> {
        let jvm = Jvm::attach_thread().unwrap();
        Ok(jvm.clone_instance(self.instance.as_ref().unwrap()).unwrap())
    }
}

impl TryFromInstanceTrait for QrCodeLoginListener {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self {
            instance: Some(instance),
            on_fetch_qrcode: None,
            on_state_changed: None,
            on_interval_loop: None,
            on_completed: None,
        })
    }
}

pub trait QrCodeLoginListenerTrait
where
    Self: TryFromInstanceTrait + GetInstanceTrait,
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

#[derive(AsInstanceDerive, GetInstanceDerive)]
pub struct SmsRequests {
    instance: Instance,
}

impl SmsRequests {
    pub fn get_country_code(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getUrl", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_phone_number(&self) -> String {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getUrl", InvocationArg::empty())
                .unwrap(),
        )
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
        jvm.to_rust(
            jvm.invoke(&self.instance, "getUrl", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn solved(&self) -> DeviceVerificationResult {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "solved", InvocationArg::empty())
            .unwrap();
        DeviceVerificationResult { instance }
    }
}

pub struct DeviceVerificationRequests {
    instance: Instance,
}

impl DeviceVerificationRequests {
    pub fn get_fallback(&self) -> FallbackRequests {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getFallback", InvocationArg::empty())
            .unwrap();
        FallbackRequests { instance }
    }
    pub fn get_prefer_sms(&self) -> bool {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.to_rust(
            jvm.invoke(&self.instance, "getPreferSms", InvocationArg::empty())
                .unwrap(),
        )
        .unwrap()
    }
    pub fn get_sms(&self) -> SmsRequests {
        let jvm = Jvm::attach_thread().unwrap();
        let instance = jvm
            .invoke(&self.instance, "getSms", InvocationArg::empty())
            .unwrap();
        SmsRequests { instance }
    }
}

impl TryFromInstanceTrait for DeviceVerificationRequests {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self { instance })
    }
}

#[derive(AsInstanceDerive, GetInstanceDerive)]
pub struct DeviceVerificationResult {
    instance: Instance,
}

impl TryFromInstanceTrait for DeviceVerificationResult {
    fn try_from_instance(instance: Instance) -> Result<Self, J4RsError> {
        Ok(Self { instance })
    }
}

pub trait LoginSolverTrait: 'static {
    const IS_SLIDER_CAPTCHA_SUPPORTED: bool = true;
    fn on_solve_slider_captcha(bot: Bot, url: &str) -> String;
    fn on_solve_pic_captcha(bot: Bot, data: &Vec<i8>) -> String;
    fn create_qrcode_login_listener(bot: Bot) -> QrCodeLoginListener;
    fn on_solve_device_verification(
        bot: Bot,
        requests: DeviceVerificationRequests,
    ) -> DeviceVerificationResult;
    fn __on_solve_slider_captcha(bot: Bot, url: JavaString) -> JavaString {
        Self::on_solve_slider_captcha(bot, url.as_str()).into()
    }
    fn __on_solve_pic_captcha(bot: Bot, data: JavaBytes) -> JavaString {
        Self::on_solve_pic_captcha(bot, data.deref()).into()
    }
    fn __create_qrcode_login_listener(bot: Bot) -> QrCodeLoginListener {
        Self::create_qrcode_login_listener(bot).into()
    }
    fn __instance() -> (
        Instance,
        Func2<Bot, JavaString, JavaString>,
        Func2<Bot, JavaBytes, JavaString>,
        Func1<Bot, QrCodeLoginListener>,
        Func2<Bot, DeviceVerificationRequests, DeviceVerificationResult>,
    ) {
        let jvm = Jvm::attach_thread().unwrap();

        let _1 = Func2::new(&Self::__on_solve_slider_captcha);
        let _2 = Func2::new(&Self::__on_solve_pic_captcha);
        let _3 = Func1::new(&Self::__create_qrcode_login_listener);
        let _4 = Func2::new(&Self::on_solve_device_verification);

        let on_solve_slider_captcha = InvocationArg::try_from(_1.get_instance()).unwrap();
        let on_solve_pic_captcha = InvocationArg::try_from(_2.get_instance()).unwrap();
        let is_slider_captcha_supported =
            InvocationArg::try_from(Self::IS_SLIDER_CAPTCHA_SUPPORTED).unwrap();
        let create_qrcode_login_listener = InvocationArg::try_from(_3.get_instance()).unwrap();
        let on_solve_device_verification = InvocationArg::try_from(_4.get_instance()).unwrap();
        (
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
            .unwrap(),
            _1,
            _2,
            _3,
            _4,
        )
    }
}
