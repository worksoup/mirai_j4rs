pub trait BotBackend: Default + Clone {}

#[derive(Default, Clone)]
pub struct Mirai;
impl BotBackend for Mirai {}

#[derive(Default, Clone)]
pub struct Overflow;

impl BotBackend for Overflow {}
