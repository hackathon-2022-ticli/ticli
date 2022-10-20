mod cli;
mod client;
mod record;
mod transcode;

use anyhow::Result;
use clap::Parser;
use cli::TiCLI;
use client::Client;
use owo_colors::OwoColorize;
use std::{io, process, str};
use tabled::{format::Format, object::Rows, Alignment, Modify, Style, TableIteratorExt};
use transcode::KvPairExt;

use crate::record::Record;

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                process::exit(0);
            }
        }

        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
        std::process::exit(1)
    }
}

async fn try_main() -> anyhow::Result<()> {
    let ticli = TiCLI::parse();

    let addr = format!("{}:{}", ticli.host, ticli.port);
    let client = match ticli.mode {
        cli::Mode::Txn => Client::txn(addr).await?,
        cli::Mode::Raw => Client::raw(addr).await?,
    };

    match ticli.command {
        cli::Command::Get { key } => {
            let value = client.get(key.clone()).await?;
            match value {
                Some(buf) => {
                    let value = str::from_utf8(&buf)?;
                    let mut table = vec![Record::new(&key, value)].table();
                    table
                        .with(
                            Modify::new(Rows::first())
                                .with(Alignment::center())
                                .with(Format::new(|s| s.bright_green().bold().to_string())),
                        )
                        .with(Style::rounded());
                    println!("{table}");
                }
                None => println!("{}", "(nil)".bright_black().italic()),
            }
        }
        cli::Command::Set { key, value } => {
            client.set(key, value).await?;
            println!("{}", "OK".bright_green());
        }
        cli::Command::Scan { prefix, limit } => {
            let kvs = client.scan(prefix, limit).await?;
            let kvs: Vec<_> = kvs.iter().map(|kv| kv.utf8()).collect::<Result<_>>()?;
            // TODO: unify the output format
            if kvs.is_empty() {
                println!("{}", "(nil)".bright_black().italic())
            } else {
                let mut table = kvs
                    .into_iter()
                    .enumerate()
                    .map(|(i, (k, v))| Record::new(k, v).indexed(i + 1))
                    .table();
                table
                    .with(
                        Modify::new(Rows::first())
                            .with(Alignment::center())
                            .with(Format::new(|s| s.bright_green().bold().to_string())),
                    )
                    .with(Style::rounded());
                println!("{table}");
            }
        }
        cli::Command::Ping {} => {
            client.get("".to_string()).await?;
            println!("{}", "pong".bright_green());
        }
    }
    Ok(())
}
