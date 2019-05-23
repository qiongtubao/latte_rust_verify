extern crate json;
use json::{JsonValue};

pub trait ConfigParser {
    fn toJson(self) -> Result<JsonValue, &'static str>;
}

impl ConfigParser for String {
    fn toJson(self) -> Result<JsonValue, &'static str> {
       Ok(json::parse(&self).unwrap())
    }
}
impl ConfigParser for JsonValue {
    fn toJson(self) -> Result<JsonValue, &'static str> {
        Ok(self)
    }
}