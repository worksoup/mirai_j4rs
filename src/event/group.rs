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
    use contact_derive::{GetClassTypeDerive, GetInstanceDerive};
    use j4rs::{Instance, InvocationArg, Jvm};
    use crate::contact::bot::Bot;
    use crate::contact::group::Group;
    use crate::contact::NormalMember;
    use crate::env::FromInstance;
    use crate::event::event_trait::{BotEventTrait, MiraiEventTrait};
    use crate::message::message_trait::MessageHashCodeTrait;

    pub enum MemberJoinEvent {
        Invite,
        Active,
    }

    pub enum MemberLeaveEvent {
        Kick,
        Quit,
    }

    // TODO: BaseGroupMemberInfoChangeEvent
    #[derive(GetInstanceDerive, GetClassTypeDerive)]
    pub struct MemberJoinRequestEvent {
        instance: Instance,
    }

    impl MemberJoinRequestEvent {
        fn get_class_name() -> String {
            "net.mamoe.mirai.event.events.MemberJoinRequestEvent".to_string()
        }
        pub fn accept(&self) {
            let jvm = Jvm::attach_thread().unwrap();
            let _ = jvm.invoke(&self.instance, "getBot", &[]).unwrap();
        }
        pub fn equals(&self, other: Self) -> bool {
            todo!()
        }
        pub fn get_bot(&self) -> Bot {
            let instance = Jvm::attach_thread().unwrap().invoke(&self.instance, "getBot", &[]).unwrap();
            Bot::from_instance(instance)
        }
        pub fn get_event_id(&self) -> i64 {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getEventId", &[]).unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_from_id(&self) -> i64 {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getFromId", &[]).unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_from_nick(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getFromNick", &[]).unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_group(&self) -> Group {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getGroup", &[]).unwrap();
            Group::from_instance(instance)
        }
        pub fn get_group_id(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getGroupId", &[]).unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_group_name(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getGroupName", &[]).unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_invitor(&self) -> NormalMember {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getInvitor", &[]).unwrap();
            NormalMember::from_instance(instance)
        }
        pub fn get_invitor_id(&self) -> i64 {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getInvitorId", &[]).unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn get_message(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "getMessage", &[]).unwrap();
            jvm.to_rust(instance).unwrap()
        }
        pub fn ignore(&self, blacklist: bool) {
            let jvm = Jvm::attach_thread().unwrap();
            let blacklist = InvocationArg::try_from(blacklist).unwrap().into_primitive().unwrap();
            let _ = jvm.invoke(&self.instance, "ignore", &[blacklist]).unwrap();
        }
        pub fn reject(&self, blacklist: bool) {
            let jvm = Jvm::attach_thread().unwrap();
            let blacklist = InvocationArg::try_from(blacklist).unwrap().into_primitive().unwrap();
            let _ = jvm.invoke(&self.instance, "reject", &[blacklist]).unwrap();
        }
        pub fn to_string(&self) -> String {
            let jvm = Jvm::attach_thread().unwrap();
            let instance = jvm.invoke(&self.instance, "toString", &[]).unwrap();
            jvm.to_rust(instance).unwrap()
        }
    }

    impl FromInstance for MemberJoinRequestEvent {
        fn from_instance(instance: Instance) -> Self {
            Self { instance }
        }
    }

    impl MessageHashCodeTrait for MemberJoinRequestEvent {}

    impl MiraiEventTrait for MemberJoinRequestEvent {}

    impl BotEventTrait for MemberJoinRequestEvent {}

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
