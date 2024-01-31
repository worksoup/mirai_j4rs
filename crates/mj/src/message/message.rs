use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref IMAGE_ID_REGEX: Regex =
        Regex::new(r#"\{[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}\}\..{3,5}"#)
            .expect("失效的正则表达式。");
}