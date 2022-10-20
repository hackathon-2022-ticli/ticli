use anyhow::Result;
use std::{
    ops::{Bound, RangeBounds},
    str,
};

use tikv_client::BoundRange;

pub trait RangeExt {
    fn to_string(&self) -> Result<String>;
    fn from_str(from: Option<String>, to: Option<String>) -> Result<Self>
    where
        Self: Sized;
}

pub trait BoundExt {
    fn from_str(s: Option<String>) -> Self
    where
        Self: Sized;
}

impl RangeExt for BoundRange {
    fn to_string(&self) -> Result<String> {
        let from = match self.start_bound() {
            Bound::Included(b) => format!("[{}", str::from_utf8(b.into())?),
            Bound::Excluded(b) => format!("({}", str::from_utf8(b.into())?),
            Bound::Unbounded => String::from("(-inf"),
        };
        let to = match self.end_bound() {
            Bound::Included(b) => format!("{}]", str::from_utf8(b.into())?),
            Bound::Excluded(b) => format!("{})", str::from_utf8(b.into())?),
            Bound::Unbounded => String::from("+inf)"),
        };
        Ok(format!("{}, {}", from, to))
    }
    fn from_str(from: Option<String>, to: Option<String>) -> Result<Self> {
        let from: Bound<_> = BoundExt::from_str(from);
        let to: Bound<_> = BoundExt::from_str(to);
        Ok((from, to).into())
    }
}

impl BoundExt for Bound<String> {
    fn from_str(s: Option<String>) -> Self {
        match s {
            Some(s) => Bound::Included(s),
            None => Bound::Unbounded,
        }
    }
}
