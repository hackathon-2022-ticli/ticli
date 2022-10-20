use anyhow::Result;

use crate::{cli::Command, client::Client, record::Record, transcode::KvPairExt};
use owo_colors::OwoColorize;
use std::str;
use tabled::{format::Format, object::Rows, Alignment, Modify, Style, TableIteratorExt};

pub async fn run_cmd(client: &Client, cmd: Command) -> Result<()> {
    match cmd {
        Command::Get { key } => {
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
        Command::Set { key, value } => {
            client.set(key, value).await?;
            println!("{}", "OK".bright_green());
        }
        Command::Scan { from, to, limit } => {
            let kvs = client.scan(from, to, limit).await?;
            let kvs: Vec<_> = kvs.iter().map(|kv| kv.utf8()).collect();
            // TODO: unify the output format
            if kvs.is_empty() {
                println!("{}", "(nil)".bright_black().italic())
            } else {
                let mut table = kvs
                    .iter()
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
        Command::Ping => {
            client.ping().await?;
            println!("{}", "PONG".bright_green());
        }
        Command::Delete { key } => {
            client.delete(key).await?;
            println!("{}", "(nil)".bright_black().italic())
        }
    };
    Ok(())
}
