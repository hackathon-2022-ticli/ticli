use tikv_client::{BoundRange, KvPair};

use crate::{
    cli::OutputFormat,
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

    pub fn from_kv(key: String, value: Option<V>) -> Self {
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

impl ScanResult {
    fn to_table(&self) -> Table {
        Table {
            header:   &["KEY", "VALUE"],
            body:     self
                .items
                .iter()
                .map(|kv| vec![kv.key().render(), kv.value().render()])
                .collect(),
            with_seq: true,
        }
    }

    pub fn render_with_format(&self, format: OutputFormat) -> String {
        self.to_table().render_with_format(format)
    }

    pub fn print_with_format(&self, format: OutputFormat) {
        println!("{}", self.render_with_format(format));
    }
}

impl Render for ScanResult {
    fn render(&self) -> String {
        self.to_table().render_with_format(OutputFormat::Auto)
    }
}

impl From<Vec<KvPair>> for ScanResult {
    fn from(items: Vec<KvPair>) -> Self {
        Self { items }
    }
}
