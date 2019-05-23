mod configParser;
mod verifyConfig;
mod verify;
mod types;
pub use verify::*;
pub use verifyConfig::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::verify::Verify;
    #[test]
    fn verify_integer() {
        let a: verifyConfig::VerifyConfig = "{\"$lt\":1}".into();
        let result = 3_i32.verify(a).unwrap();
        assert_eq!(result, true);
        let a: verifyConfig::VerifyConfig = "{\"$gt\":5}".into();
        match 3_i32.verify(a) {
            Ok(v) => {}
            Err(err) => {
                assert_eq!(err, "min error");
            }
        }
    }

    #[test]
    fn verify_string() {
        let result = "123".verify("{\"len\":3}".into()).unwrap();
        assert_eq!(result, true);
    }
}