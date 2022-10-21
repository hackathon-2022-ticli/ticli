use std::{
    fs::File,
    io::{self, Read},
};

use anyhow::Result;
use async_recursion::async_recursion;
use tikv_client::{BoundRange, KvPair};

use crate::{
    cli::Command,
    client::Client,
    format::{table::Table, Literal::*},
    parser,
    range::BoundRangeExt,
};

#[async_recursion(?Send)]
pub async fn execute(client: &Client, cmd: Command) -> Result<()> {
    match cmd {
        Command::Get { key } => {
            let value = client.get(key.clone()).await?;
            let kv_pair = value.map(|value| KvPair::new(key, value));
            let table: Table = kv_pair.into();
            println!("{}", table.format());
        }
        Command::Set { key, value } => {
            client.set(key, value).await?;
            println!("{}", OK.format());
        }
        Command::Scan { from, to, limit } => {
            let kvs = client.scan(from, to, limit).await?;
            let table: Table = kvs.into();
            println!("{}", table.with_seq().format());
        }
        Command::Count { from, to } => {
            let range: BoundRange = BoundRangeExt::build(from, to);
            let count = client.count(range.clone()).await?;
            let rows = vec![vec![range.to_string(), count.to_string()]];
            let table = Table::new(&["RANGE", "COUNT"], rows);
            println!("{}", table.format());
        }
        Command::Delete { key } => {
            client.delete(key).await?;
            println!("{}", OK.format());
        }
        Command::Source { file } => {
            let file: Box<dyn Read> = match file {
                Some(file) => Box::new(File::open(file)?),
                None => Box::new(io::stdin()),
            };
            for cmd in parser::from_reader(file) {
                execute(client, cmd?).await?;
            }
        }
        Command::Strlen { key } => {
            let len = client.strlen(key.clone()).await?;
            let row = len.map(|len| vec![vec![key, len.to_string()]]);
            let table = Table::new(&["KEY", "LENGTH"], row.unwrap_or_default());
            println!("{}", table.format());
        }
        Command::Ping => {
            client.ping().await?;
            println!("{}", PONG.format());
        }
        Command::Quit => {
            println!("{}", Goodbye.format());
            std::process::exit(0);
        }
        Command::Noop => {}
    };
    Ok(())
}
