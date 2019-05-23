use json::{JsonValue};
use super::configParser::*;
pub enum VerifyConfig {
    String(String)
}
impl From<String> for VerifyConfig {
    fn from(item: String) -> Self {
        VerifyConfig::String(item)
    }
}
impl From<&'static str> for VerifyConfig {
    fn from(st: &'static str) -> Self {
        VerifyConfig::String(st.to_string())
    }
}

pub fn configParse(v: VerifyConfig) -> Result<JsonValue, &'static str> {
    match v {
        VerifyConfig::String(encoded) => {
            encoded.toJson()
        }
        _ => {
            return Err("config Err");
        }
    }
}