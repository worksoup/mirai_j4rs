use contact_derive::GetInstanceDerive;
use j4rs::{prelude::*, InvocationArg};
use j4rs::{Instance, Jvm};
use j4rs_derive::*;
use std::mem::transmute;

use crate::env::{GetClassTypeTrait, GetEnvTrait};

//需要由Env构造。
pub struct EventChannel<E>
where
    E: MiraiEventTrait,
{
    pub(crate) jvm: Jvm,
    pub(crate) instance: Instance,
    pub(crate) _unused: Option<E>,
}

#[call_from_java("rt.lea.Lumia.onEvent")]
fn apply_on_event(on_event_ptr: Instance, event: Instance) {
    let on_event_raw: [i8; 16] = Jvm::attach_thread().unwrap().to_rust(on_event_ptr).unwrap();
    println!("rust side 2");
    println!("{:?}", on_event_raw);
    let on_event: *mut dyn Fn(AbstractEvent) -> () = unsafe { transmute(on_event_raw) };
    unsafe {
        let _ = (*on_event)(AbstractEvent::from_instance(event));
    };
}

impl<'a, E> EventChannel<E>
where
    E: MiraiEventTrait,
{
    fn subscribe_internal(
        &self,
        call_from_java_raw_list: [i8; 16],
    ) -> (Instance, Instance, [i8; 16]) {
        println!("rust side 1");
        println!("{:?}", call_from_java_raw_list);
        let mut on_event_ptr = Vec::new();
        for i in call_from_java_raw_list {
            on_event_ptr.push(InvocationArg::try_from(i).unwrap());
        }
        let class_type = E::get_class_type();
        let on_event_ptr = self
            .jvm
            .create_java_array("java.lang.Byte", &on_event_ptr)
            .unwrap();
        let consumer = Jvm::attach_thread()
            .unwrap()
            .create_instance(
                "rt.lea.Lumia",
                &[InvocationArg::try_from(on_event_ptr).unwrap()],
            )
            .unwrap();
        (class_type, consumer, call_from_java_raw_list)
    }
    fn subscribe_internal_0_1(on_event: &'a Box<dyn Fn(E) -> ()>) -> [i8; 16] {
        let call_from_java: Box<dyn Fn(AbstractEvent) -> ()> = Box::new(|e: AbstractEvent| {
            let e: E = e.get::<E>();
            on_event(e);
        });
        let call_from_java_raw: *mut dyn Fn(AbstractEvent) = Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    fn subscribe_internal_0_2(on_event: Box<dyn FnOnce(E) -> ()>) -> [i8; 16] {
        let call_from_java: Box<dyn FnOnce(AbstractEvent) -> ()> =
            Box::new(move |e: AbstractEvent| {
                let e: E = e.get::<E>();
                on_event(e);
            });
        let call_from_java_raw: *mut dyn FnOnce(AbstractEvent) = Box::into_raw(call_from_java);
        unsafe { transmute::<_, [i8; 16]>(call_from_java_raw) }
    }
    fn subscribe_internal_1_1(
        &self,
        on_event: &Box<dyn Fn(E) -> ()>,
    ) -> (Instance, Instance, [i8; 16]) {
        let call_from_java_raw_list = Self::subscribe_internal_0_1(on_event);
        self.subscribe_internal(call_from_java_raw_list)
    }
    fn subscribe_internal_1_2(
        &self,
        on_event: Box<dyn FnOnce(E) -> ()>,
    ) -> (Instance, Instance, [i8; 16]) {
        let call_from_java_raw_list = Self::subscribe_internal_0_2(on_event);
        self.subscribe_internal(call_from_java_raw_list)
    }
    pub fn subscribe(&'a self, on_event: &'a Box<dyn Fn(E) -> ()>) -> Listener<E> {
        let (class_type, consumer, call_from_java) = self.subscribe_internal_1_1(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribe",
                &[
                    InvocationArg::try_from(class_type).unwrap(),
                    InvocationArg::try_from(consumer).unwrap(),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            call_from_java,
            _on_event: OnEvent::Fn(on_event),
        }
    }
    pub fn subscribe_always(&'a self, on_event: &'a Box<dyn Fn(E) -> ()>) -> Listener<E> {
        let (class_type, consumer, call_from_java) = self.subscribe_internal_1_1(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribeAlways",
                &[
                    InvocationArg::try_from(class_type).unwrap(),
                    InvocationArg::try_from(consumer).unwrap(),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            call_from_java,
            _on_event: OnEvent::Fn(on_event),
        }
    }
    pub fn subscribe_once(&self, on_event: Box<dyn FnOnce(E) -> ()>) -> Listener<E> {
        let (class_type, consumer, call_from_java) = self.subscribe_internal_1_2(on_event);
        let listener = Jvm::attach_thread()
            .unwrap()
            .invoke(
                &self.instance,
                "subscribeOnce",
                &[
                    InvocationArg::try_from(class_type).unwrap(),
                    InvocationArg::try_from(consumer).unwrap(),
                ],
            )
            .unwrap();
        Listener {
            instance: listener,
            call_from_java,
            _on_event: OnEvent::FnOnce,
        }
    }
    pub fn exception_handler(&self) -> Self {
        todo!()
    }
    pub fn filter(&self) -> Self {
        todo!()
    }
}

pub trait MiraiEventTrait
where
    Self: GetEnvTrait + GetClassTypeTrait,
{
    fn from_instance(instance: Instance) -> Self;
    fn cancel(&self);
    fn intercept(&self);
    fn is_canceled(&self) -> bool;
    fn is_intercepted(&self) -> bool;
    fn broadcast(&self);
}

pub enum OnEvent<'a, E> {
    Fn(&'a Box<dyn Fn(E)>),
    // 此处需要值，确保引用有效，值不会被 drop.
    FnOnce, // 此处不需要值，因为值已经移动到下方 Listener 中 call_from_java 这个指针所代表的值里了。
}

pub struct Listener<'a, E> {
    instance: Instance,
    call_from_java: [i8; 16],
    _on_event: OnEvent<'a, E>,
}

impl<E> Listener<'_, E> {
    pub fn cancel(self) {
        todo!()
    }
    pub fn complete(self) -> bool {
        let call_from_java: *mut dyn Fn(AbstractEvent) -> () =
            unsafe { transmute(self.call_from_java) };
        let call_from_java = unsafe { Box::from_raw(call_from_java) };
        drop(call_from_java);
        let jvm = Jvm::attach_thread().unwrap();
        let b = jvm.invoke(&self.instance, "complete", &[]).unwrap();
        jvm.to_rust(b).unwrap()
    }
}

#[derive(GetInstanceDerive)]
pub struct AbstractEvent {
    instance: Instance,
}

impl AbstractEvent {
    pub fn get<E>(&self) -> E
    where
        E: MiraiEventTrait,
    {
        E::from_instance(
            Jvm::attach_thread()
                .unwrap()
                .clone_instance(&self.instance)
                .unwrap(),
        )
    }
}

impl GetClassTypeTrait for AbstractEvent {
    fn get_class_type() -> Instance {
        panic!("本 api 不应当使用。")
    }
}

impl MiraiEventTrait for AbstractEvent {
    fn from_instance(instance: Instance) -> Self {
        Self { instance }
    }

    fn cancel(&self) {
        todo!()
    }

    fn intercept(&self) {
        todo!()
    }

    fn is_canceled(&self) -> bool {
        todo!()
    }

    fn is_intercepted(&self) -> bool {
        todo!()
    }

    fn broadcast(&self) {
        todo!()
    }
}

pub mod bot {
    use contact_derive::GetInstanceDerive;
    use j4rs::{Instance, Jvm};

    use crate::{contact::bot::Bot, env::GetClassTypeTrait};

    use super::MiraiEventTrait;

    pub trait BotEventTrait
    where
        Self: MiraiEventTrait,
    {
        fn get_bot(&self) -> Bot;
    }

    #[derive(GetInstanceDerive)]
    pub struct BotOnlineEvent {
        instance: Instance,
    }

    impl GetClassTypeTrait for BotOnlineEvent {
        fn get_class_type() -> Instance {
            todo!()
        }
    }

    impl MiraiEventTrait for BotOnlineEvent {
        fn from_instance(instance: Instance) -> Self {
            todo!()
        }
        fn cancel(&self) {
            let _ = Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "cancel", &[]);
        }

        fn intercept(&self) {
            let _ = Jvm::attach_thread()
                .unwrap()
                .invoke(&self.instance, "intercept", &[]);
        }

        fn is_canceled(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(&self.instance, "isCanceled", &[])
                        .unwrap(),
                )
                .unwrap()
        }

        fn is_intercepted(&self) -> bool {
            Jvm::attach_thread()
                .unwrap()
                .to_rust(
                    Jvm::attach_thread()
                        .unwrap()
                        .invoke(&self.instance, "isIntercepted", &[])
                        .unwrap(),
                )
                .unwrap()
        }

        fn broadcast(&self) {
            todo!()
        }
    }

    impl BotEventTrait for BotOnlineEvent {
        fn get_bot(&self) -> Bot {
            todo!()
        }
    }

    pub trait BotOfflineEventTrait {}

    pub struct Active {}

    pub struct Force {}

    pub struct Dropped {}

    pub struct RequireReconnect {}

    pub enum BotOfflineEvent {
        Active(Active),
        Force(Force),
        Dropped(Dropped),
        RequireReconnect(RequireReconnect),
    }

    impl BotOfflineEventTrait for BotOfflineEvent {}

    pub struct BotReloginEvent {}

    pub struct BotAvatarChangedEvent {}

    pub struct BotNickChangedEvent {}

    pub struct NudgeEvent {}
}

pub mod message {
    use super::MiraiEventTrait;
    use crate::{
        contact::{bot::Bot, group::Group, AnonymousMember, Friend, Member, NormalMember},
        message::MessageChain,
    };
    use contact_derive::{GetClassTypeDerive, GetInstanceDerive};
    use j4rs::{Instance, Jvm};

    pub trait MessageEventTrait
    where
        Self: MiraiEventTrait,
    {
        fn get_bot(&self) -> Bot {
            todo!()
        }
        fn get_message(&self) -> MessageChain {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.get_instance(), "getMessage", &[]).unwrap();
            MessageChain { instance }
        }
        type UserItem;
        fn get_sender(&self) -> Self::UserItem;
        fn get_sender_name(&self) -> String {
            todo!()
        }
        fn get_source(&self) {
            todo!()
        }
        type ContactItem;
        fn get_subject(&self) -> Self::ContactItem;
        fn get_time(&self) -> i64 {
            todo!()
        }
    }

    #[derive(GetInstanceDerive, GetClassTypeDerive)]
    pub struct GroupMessageEvent {
        instance: Instance,
    }

    impl GroupMessageEvent {
        // 该函数被 GetClassTypeDerive 宏使用。该宏实现了 GetClassTypeTrait。
        // 这个特征可以返回 java 中 Class 对象，监听事件的时候用。
        // 为了做泛型搞的。之后可能会改动。
        /// 获取 java 中的类名。TODO: 需要移除该函数。该函数的引入是由于 j4rs 旧版本中的 bug.
        /// `getClass` 方法属于每一个 Object, 但由于 bug, 无法通过 j4rs 直接调用之。
        /// 见 https://github.com/astonbitecode/j4rs/issues/71
        fn get_class_name() -> String {
            "net.mamoe.mirai.event.events.GroupMessageEvent".to_owned()
        }
    }

    // 这个特征实现了一个 event 所需要的函数。
    impl MiraiEventTrait for GroupMessageEvent {
        fn from_instance(instance: Instance) -> Self {
            GroupMessageEvent { instance }
        }

        fn cancel(&self) {
            todo!()
        }

        fn intercept(&self) {
            todo!()
        }

        fn is_canceled(&self) -> bool {
            todo!()
        }

        fn is_intercepted(&self) -> bool {
            todo!()
        }

        fn broadcast(&self) {
            todo!()
        }
    }

    // 实现了 message 所需要的函数。
    impl MessageEventTrait for GroupMessageEvent {
        type ContactItem = Group;

        fn get_subject(&self) -> Self::ContactItem {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getSubject", &[]).unwrap();
            let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
            let id = jvm
                .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
                .unwrap();
            Group { bot, instance, id }
        }

        type UserItem = Member;

        fn get_sender(&self) -> Self::UserItem {
            // j4rs 旧版本中有 bug, 所以只能如下注释中的写法。见 https://github.com/astonbitecode/j4rs/issues/71
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getSender", &[]).unwrap();
            let bot = jvm
                .invoke(
                    // &jvm.cast(&instance, "net.mamoe.mirai.contact.Contact")             // j4rs <= 0.17.1
                    // .unwrap(),                                                          // j4rs <= 0.17.1
                    &instance,
                    "getBot",
                    &[],
                )
                .unwrap();
            let id = jvm
                .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
                .unwrap();
            let spevial_title: String = jvm
                .to_rust(
                    jvm.invoke(
                        // 下面两行之所以转换是因为 java 中这个函数似乎返回了 `net.mamoe.mirai.contact.User`, 是没有 `getSpecialTitle` 这个函数的。
                        &jvm.cast(&instance, "net.mamoe.mirai.contact.Member")
                            .unwrap(),
                        // &instance,
                        "getSpecialTitle",
                        &[],
                    )
                    .unwrap(),
                )
                .unwrap();
            match spevial_title.as_str() {
                "匿名" => {
                    println!("匿名成员");
                    Member::AnonymousMember(AnonymousMember { bot, instance, id })
                }
                _ => {
                    println!("普通成员");
                    Member::NormalMember(NormalMember { bot, instance, id })
                }
            }
        }
    }

    #[derive(GetInstanceDerive, GetClassTypeDerive)]
    pub struct FriendMessageEvent {
        instance: Instance,
    }

    impl FriendMessageEvent {
        fn get_class_name() -> String {
            "net.mamoe.mirai.event.events.FriendMessageEvent".to_owned()
        }
    }

    impl MiraiEventTrait for FriendMessageEvent {
        fn from_instance(instance: Instance) -> Self {
            FriendMessageEvent { instance }
        }

        fn cancel(&self) {
            todo!()
        }

        fn intercept(&self) {
            todo!()
        }

        fn is_canceled(&self) -> bool {
            todo!()
        }

        fn is_intercepted(&self) -> bool {
            todo!()
        }

        fn broadcast(&self) {
            todo!()
        }
    }

    impl MessageEventTrait for FriendMessageEvent {
        type ContactItem = Friend;
        type UserItem = Friend;
        fn get_subject(&self) -> Friend {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getSubject", &[]).unwrap();
            let bot = jvm.invoke(&instance, "getBot", &[]).unwrap();
            let id = jvm
                .to_rust(jvm.invoke(&instance, "getId", &[]).unwrap())
                .unwrap();
            Friend { bot, instance, id }
        }

        fn get_sender(&self) -> Self::UserItem {
            todo!()
        }
    }

    pub struct GroupTempMessageEvent {}

    pub struct StrangerMessageEvent {}

    pub struct OtherClientMessageEvent {}

    pub trait MessagePreSendEventTrait {}

    pub struct GroupMessagePreSendEvent {}

    pub struct FriendMessagePreSendEvent {}

    pub struct GroupTempMessagePreSendEvent {}

    pub struct StrangerMessagePreSendEvent {}

    pub struct OtherClientMessagePreSendEvent {}

    pub trait MessagePostSendEventTrait {}

    pub struct GroupMessagePostSendEvent {}

    pub struct FriendMessagePostSendEvent {}

    pub struct GroupTempMessagePostSendEvent {}

    pub struct StrangerMessagePostSendEvent {}

    pub struct OtherClientMessagePostSendEvent {}

    pub trait MessageRecallTrait {}

    pub enum MessageRecall {
        FriendRecall,
        GroupRecall,
        TempRecall,
    }

    pub struct BeforeImageUploadEvent {}

    pub enum ImageUploadEvent {
        Succeed,
        Failed,
    }

    pub struct NudgeEvent {}

    pub trait MessageSyncEvent {}
}

pub mod group {
    pub enum BotLeaveEvent {
        Active,
        Kick,
    }

    pub struct BotGroupPermissionChangeEvent {}

    pub struct BotMuteEvent {}

    pub struct BotUnmuteEvent {}

    pub struct BotJoinGroupEvent {}

    pub mod settings {
        pub trait GroupSettingsChangeEvent {}

        pub struct GroupNameChangeEvent {}

        pub struct GroupEntranceAnnouncementChangeEvent {}

        pub struct GroupMuteAllChangeEvent {}

        pub struct GroupAllowAnonymousChatChangeEvent {}

        pub struct GroupAllowMemberInviteChangeEvent {}
    }

    pub mod member {
        pub enum MemberJoinEvent {
            Invite,
            Active,
        }

        pub enum MemberLeaveEvent {
            Kick,
            Quit,
        }

        pub struct MemberJoinRequestEvent {}

        pub struct BotInvitedJoinGroupRequestEvent {}
    }

    pub mod honor {
        pub struct MemberCardChangeEvent {}

        pub struct MemberSpecialTitleChangeEvent {}
    }

    pub mod member_permission {
        pub struct MemberPermissionChangeEvent {}
    }

    pub mod action {
        pub struct MemberMuteEvent {}

        pub struct MemberUnmuteEvent {}
    }
}

pub mod friend {
    pub struct FriendRemarkChangeEvent {}

    pub struct FriendAddEvent {}

    pub struct FriendDeleteEvent {}

    pub struct NewFriendRequestEvent {}

    pub struct FriendAvatarChangedEvent {}

    pub struct FriendNickChangedEvent {}

    pub struct FriendInputStatusChangedEvent {}
}
