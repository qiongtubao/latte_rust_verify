extern crate regex;
use regex::Regex;
use super::*;
use super::verify::{Verify};
use super::verifyConfig::{self,VerifyConfig};
impl Verify for &'static str {
    fn verify(self, v: VerifyConfig) -> Result<bool, &'static str> {
        self.to_string().verify(v)
    }
}
impl Verify for String {
    fn verify(self, v: VerifyConfig) -> Result<bool, &'static str> {
        let mut decoded :json::JsonValue = verifyConfig::configParse(v)?;
        // println!("decoded: {:?}", decoded["$regex"]);
        match decoded["$regex"].clone() {
            json::JsonValue::Short(c) => {
                let regexStr = c.to_string();
                let re = Regex::new(&regexStr).unwrap();
                if !re.is_match(&self) {
                    return Err("$regex error");
                }
            },
            _ => {}
        }
        Ok(true)
    }
}