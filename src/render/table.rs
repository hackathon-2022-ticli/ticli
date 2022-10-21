use std::{collections::BTreeMap, iter};

use crate::cli::OutputFormat;

use super::{is_tty, Literal::NIL, Render};
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

            self.body.iter().enumerate().for_each(|(i, _)| {
                builder.add_record(self.actual_row(i));
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
    pub fn render_with_format(&self, format: OutputFormat) -> String {
        match format {
            OutputFormat::Table => self.render(),
            OutputFormat::Json => self.render_json(),
            OutputFormat::Csv => self.render_csv(),
            OutputFormat::Auto => match is_tty() {
                true => self.render_with_format(OutputFormat::Table),
                false => self.render_with_format(OutputFormat::Csv),
            },
        }
    }

    pub fn render_csv(&self) -> String {
        let mut buf = Vec::new();
        {
            let mut wtr = csv::Writer::from_writer(&mut buf);
            wtr.write_record(self.actual_header()).expect("write header to buffer");
            self.body.iter().enumerate().for_each(|(i, _)| {
                wtr.write_record(self.actual_row(i)).expect("write row to buffer");
            });
        }
        buf.pop(); // remove trailing newline
        String::from_utf8(buf).expect("convert buffer to string")
    }

    pub fn render_json(&self) -> String {
        let m = self.body.iter().map(|row| {
            self.header
                .iter()
                .map(|s| s.to_lowercase())
                .zip(row)
                .collect::<BTreeMap<_, _>>()
        });
        let value = serde_json::to_value(m.collect::<Vec<_>>()).expect("convert to json value");
        colored_json::to_colored_json_auto(&value).expect("cannot convert to json string")
    }

    fn actual_header(&self) -> Vec<&str> {
        match self.with_seq {
            true => iter::once("#").chain(self.header.iter().copied()).collect(),
            false => self.header.to_vec(),
        }
    }

    fn actual_row(&self, i: usize) -> Vec<String> {
        match self.with_seq {
            true => iter::once((i + 1).to_string())
                .chain(self.body[i].iter().cloned())
                .collect(),
            false => self.body[i].to_vec(),
        }
    }
}
