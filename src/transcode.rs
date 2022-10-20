use owo_colors::OwoColorize;
use tikv_client::KvPair;

pub trait KvPairExt {
    fn utf8(&self) -> (String, String);
}

impl KvPairExt for KvPair {
    fn utf8(&self) -> (String, String) {
        let key: &[u8] = self.key().into();
        let value: &[u8] = self.value();
        (string_or_bytes(key), string_or_bytes(value))
    }
}

fn string_or_bytes(buf: &[u8]) -> String {
    match std::str::from_utf8(buf) {
        Ok(s) => s.to_string(),
        Err(_) => format!("{}", base64::encode(buf).purple()),
    }
}
