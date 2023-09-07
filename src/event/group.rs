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
