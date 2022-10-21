use std::io::{BufRead, BufReader, Read};

use anyhow::{bail, Result};
use clap::Parser;

use crate::cli::{Command, TiCLI};

pub fn from_reader(reader: impl Read) -> impl Iterator<Item = Result<Command>> {
    BufReader::new(reader).lines().map(|line| {
        line.map_err(|e| e.into()).and_then(|line| match shlex::split(&line) {
            Some(args) => {
                let args = std::iter::once("".to_string()).chain(args);
                match TiCLI::try_parse_from(args) {
                    Ok(TiCLI { command: Some(command), .. }) => Ok(command),
                    Ok(TiCLI { command: None, .. }) => Ok(Command::Noop),
                    Err(e) => bail!(e),
                }
            }
            None => bail!("invalid quoting"),
        })
    })
}
