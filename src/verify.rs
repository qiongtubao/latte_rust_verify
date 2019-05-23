
use super::verifyConfig::VerifyConfig;
pub trait Verify {
    fn verify(self, v :VerifyConfig) -> Result<bool, &'static str>;
}