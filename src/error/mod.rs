use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MiraiRsError {
    title: String,
    what: String,
}

impl Display for MiraiRsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(("[".to_string() + self.title.as_str() + "]: " + self.what.as_str()).as_str())
    }
}

impl Error for MiraiRsError {}
