use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct MiraiRsError {
    pub(crate) r#type: String,
    pub(crate) what: String,
}

impl Display for MiraiRsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(("[".to_string() + self.r#type.as_str() + "]: " + self.what.as_str()).as_str())
    }
}

impl Error for MiraiRsError {}
