use std::ops::{Bound, RangeBounds};

use tikv_client::BoundRange;

pub trait BoundRangeExt {
    fn to_string(&self) -> String;
    fn build(from: Option<String>, to: Option<String>) -> Self
    where
        Self: Sized;
}

pub trait BoundExt {
    fn build(s: Option<String>) -> Self
    where
        Self: Sized;
}

impl BoundRangeExt for BoundRange {
    fn to_string(&self) -> String {
        let from = match self.start_bound() {
            Bound::Included(b) => format!("[{}", String::from_utf8_lossy(b.into())),
            Bound::Excluded(b) => format!("({}", String::from_utf8_lossy(b.into())),
            Bound::Unbounded => String::from("(-inf"),
        };
        let to = match self.end_bound() {
            Bound::Included(b) => format!("{}]", String::from_utf8_lossy(b.into())),
            Bound::Excluded(b) => format!("{})", String::from_utf8_lossy(b.into())),
            Bound::Unbounded => String::from("+inf)"),
        };
        format!("{}, {}", from, to)
    }
    fn build(from: Option<String>, to: Option<String>) -> Self {
        let from: Bound<_> = BoundExt::build(from);
        let to: Bound<_> = BoundExt::build(to);
        (from, to).into()
    }
}

impl BoundExt for Bound<String> {
    fn build(s: Option<String>) -> Self {
        match s {
            Some(s) => Bound::Included(s),
            None => Bound::Unbounded,
        }
    }
}
