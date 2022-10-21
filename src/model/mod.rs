use tikv_client::{BoundRange, KvPair};

use crate::{
    render::{is_tty, Literal::*, Render, Table},
    tikv::BoundRangeExt,
};

pub struct KVResult<V> {
    pub key:    String,
    pub value:  Option<V>,
    pub header: &'static [&'static str; 2],
}

impl<V> KVResult<V> {
    pub fn new(header: &'static [&'static str; 2], key: String, value: Option<V>) -> Self {
        Self { header, key, value }
    }

    pub fn from_get(key: String, value: Option<V>) -> Self {
        Self::new(&["KEY", "VALUE"], key, value)
    }

    pub fn from_count(key: BoundRange, value: Option<V>) -> Self {
        Self::new(&["RANGE", "COUNT"], key.to_string(), value)
    }

    pub fn from_strlen(key: String, value: Option<V>) -> Self {
        Self::new(&["KEY", "LENGTH"], key, value)
    }
}

impl<V: Render> Render for KVResult<V> {
    fn render(&self) -> String {
        match is_tty() {
            true => {
                let row = self.value.as_ref().map(|v| vec![vec![self.key.clone(), v.render()]]);
                let table = Table::new(self.header, row.unwrap_or_default());
                table.render()
            }
            false => match &self.value {
                Some(v) => v.render(),
                None => NIL.render(),
            },
        }
    }
}

pub struct ScanResult {
    pub items: Vec<KvPair>,
}

impl Render for ScanResult {
    fn render(&self) -> String {
        let table = Table {
            header:   &["KEY", "VALUE"],
            body:     self
                .items
                .iter()
                .map(|kv| vec![kv.key().render(), kv.value().render()])
                .collect(),
            with_seq: true,
        };
        match is_tty() {
            true => table.render(),
            false => table.render_csv(),
        }
    }
}

impl From<Vec<KvPair>> for ScanResult {
    fn from(items: Vec<KvPair>) -> Self {
        Self { items }
    }
}
