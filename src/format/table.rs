use std::iter;

use super::Literal::NIL;
use owo_colors::OwoColorize;
use tabled::{builder::Builder as TableBuilder, format::Format, object::Rows, Alignment, Modify, Style};
use tikv_client::KvPair;

pub struct Table {
    pub header:   &'static [&'static str],
    pub body:     Vec<Vec<String>>,
    pub with_seq: bool,
}

impl Table {
    pub fn new(header: &'static [&'static str], body: Vec<Vec<String>>) -> Self {
        Self { header, body, with_seq: false }
    }

    pub fn format(self) -> String {
        if self.body.is_empty() {
            NIL.format()
        } else {
            let mut builder = TableBuilder::new();
            let header = match self.with_seq {
                true => iter::once("#").chain(self.header.iter().copied()).collect(),
                false => self.header.to_vec(),
            };
            builder.set_columns(header);

            for (i, row) in self.body.into_iter().enumerate() {
                let row = match self.with_seq {
                    true => iter::once((i + 1).to_string()).chain(row.into_iter()).collect(),
                    false => row,
                };
                builder.add_record(row);
            }

            builder
                .build()
                .with(
                    Modify::new(Rows::first())
                        .with(Alignment::center())
                        .with(Format::new(|s| s.bright_green().bold().to_string())),
                )
                .with(Style::rounded())
                .to_string()
        }
    }

    pub fn with_seq(mut self) -> Self {
        self.with_seq = true;
        self
    }
}

impl From<Vec<KvPair>> for Table {
    fn from(kvs: Vec<KvPair>) -> Self {
        let body = kvs
            .into_iter()
            .map(|kv| vec![string_or_base64(kv.key().into()), string_or_base64(kv.value())])
            .collect();
        Self { header: &["KEY", "VALUE"], body, with_seq: false }
    }
}

impl From<Option<KvPair>> for Table {
    fn from(kv: Option<KvPair>) -> Self {
        kv.into_iter().collect::<Vec<_>>().into()
    }
}

fn string_or_base64(buf: &[u8]) -> String {
    match std::str::from_utf8(buf) {
        Ok(s) => s.to_string(),
        Err(_) => format!("{}", base64::encode(buf).purple()),
    }
}
