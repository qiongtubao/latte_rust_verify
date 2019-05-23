use super::*;
use super::verify::{Verify};
use super::verifyConfig::{self,VerifyConfig};
macro_rules! verify_integer {
    ($ty:ty) => (
        impl Verify for $ty {
           fn verify(self, v: VerifyConfig) -> Result<bool, &'static str> {
                let mut decoded :json::JsonValue = verifyConfig::configParse(v)?;
                let ifn = |name| {
                    match &decoded[name] {
                        &json::JsonValue::Number(v) => {
                            let value: $ty = v.into();
                            match name {
                                "$gt" => { //大于
                                    if !(value < self) {
                                        return Err("$gt error");
                                    }
                                }
                                "$gte" => {
                                    if !(value <= self) {
                                        return Err("$gte error");
                                    }
                                }
                                "$lt" => {
                                    if !(value > self) {
                                        return Err("$lt error");
                                    }
                                }
                                "$lte" => {
                                    if !(value >= self) {
                                        return Err("$lte error");
                                    }
                                }
                                "$eq" => {
                                    if !(value == self) {
                                        return Err("$eq error");
                                    }
                                }
                                "ne" => {
                                    if !(value != self) {
                                        return Err("$ne error");
                                    }
                                }
                                _ => {

                                }
                            }
            
                        }
                        json::JsonValue::Array(array) => {
                            // println!("array: {:?}", array);
                            for item in array {
                                match item {
                                    &json::JsonValue::Number(v) => {
                                        let value: $ty= v.into();
                                        match name {
                                            "$in" => {
                                                if value == self {
                                                    return Ok(true);
                                                }
                                            }
                                            "$nin" => {
                                                if value == self {
                                                    return Err("$nin error");
                                                }
                                            }
                                            _ => {

                                            }
                                        }
                                    }
                                    _ => {

                                    }
                                }
                            }
                            match name {
                                "$in" => {
                                    return Err("$in error");
                                }
                                _ => {

                                }
                            }

                        }
                        _ => {
                            
                        }
                    }
                    Ok(true)
                };
                ifn("$gt")?;
                ifn("$gte")?;
                ifn("$lt")?;
                ifn("$lte")?;
                ifn("$ne")?;
                ifn("$in")?;
                ifn("$nin")?;
                Ok(true)
           }
        }
    )
}
verify_integer!(i8);
verify_integer!(i16);
verify_integer!(i32);
verify_integer!(i64);
verify_integer!(u8);
verify_integer!(u16);
verify_integer!(u32);
verify_integer!(u64);

