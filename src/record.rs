use tabled::Tabled;

#[derive(Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct Record<'a> {
    pub key:   &'a str,
    pub value: &'a str,
}

impl<'a> Record<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }

    pub fn indexed(self, idx: usize) -> IndexedRecord<'a> {
        IndexedRecord { idx, record: self }
    }
}

#[derive(Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct IndexedRecord<'a> {
    #[tabled(rename = "#")]
    pub idx:    usize,
    #[tabled(inline)]
    pub record: Record<'a>,
}
