pub trait BotBackend {}

pub struct Mirai;
impl BotBackend for Mirai {}

pub struct Overflow;

impl BotBackend for Overflow {}
