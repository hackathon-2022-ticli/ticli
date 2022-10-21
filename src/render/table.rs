use std::iter;

use super::{Literal::NIL, Render};
use owo_colors::OwoColorize;
use tabled::{
    builder::Builder as TableBuilder,
    format::Format,
    object::{Rows, Segment},
    Alignment,
    Modify,
    Style,
};

pub struct Table {
    pub header:   &'static [&'static str],
    pub body:     Vec<Vec<String>>,
    pub with_seq: bool,
}

impl Table {
    pub fn new(header: &'static [&'static str], body: Vec<Vec<String>>) -> Self {
        Self { header, body, with_seq: false }
    }
}

impl Render for Table {
    fn render(&self) -> String {
        if self.body.is_empty() {
            NIL.render()
        } else {
            let mut builder = TableBuilder::new();
            let header = self.actual_header();
            builder.set_columns(header);

            self.body.iter().enumerate().for_each(|(i, row)| {
                builder.add_record(self.actual_row(i, row));
            });

            let mut table = builder.build();
            table.with(Style::rounded()).with(
                Modify::new(Rows::first())
                    .with(Alignment::center())
                    .with(Format::new(|s| s.bright_green().bold().to_string())),
            );
            if table.count_rows() <= 2 {
                table.with(Modify::new(Segment::all()).with(Alignment::center()));
            }
            table.to_string()
        }
    }
}

impl Table {
    pub fn render_csv(&self) -> String {
        let mut buf = Vec::new();
        {
            let mut wtr = csv::Writer::from_writer(&mut buf);
            wtr.write_record(self.actual_header()).expect("write header to buffer");
            self.body.iter().enumerate().for_each(|(i, row)| {
                wtr.write_record(self.actual_row(i, row)).expect("write row to buffer");
            });
        }
        buf.pop(); // remove trailing newline
        String::from_utf8(buf).expect("convert buffer to string")
    }

    fn actual_header(&self) -> Vec<&str> {
        match self.with_seq {
            true => iter::once("#").chain(self.header.iter().copied()).collect(),
            false => self.header.to_vec(),
        }
    }

    fn actual_row(&self, i: usize, row: &[String]) -> Vec<String> {
        match self.with_seq {
            true => iter::once((i + 1).to_string()).chain(row.iter().cloned()).collect(),
            false => row.to_vec(),
        }
    }
}
