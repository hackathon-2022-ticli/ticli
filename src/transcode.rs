use anyhow::Result;
use std::str;
use tikv_client::KvPair;

pub trait KvPairExt {
    fn utf8(&self) -> Result<(&str, &str)>;
}

impl KvPairExt for KvPair {
    fn utf8(&self) -> Result<(&str, &str)> {
        let key: &[u8] = self.key().into();
        let value: &[u8] = self.value();
        Ok((std::str::from_utf8(key)?, std::str::from_utf8(value)?))
    }
}
