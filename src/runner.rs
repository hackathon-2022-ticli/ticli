use anyhow::Result;
use tikv_client::{BoundRange, KvPair};

use crate::{
    cli::Command,
    client::Client,
    format::{table::Table, Literal::*},
    range::RangeExt,
};

pub async fn run_cmd(client: &Client, cmd: Command) -> Result<()> {
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
            let range: BoundRange = RangeExt::from_str(from, to)?;
            let count = client.count(range.clone()).await?;
            let rows = vec![vec![range.to_string()?, count.to_string()]];
            let table = Table::new(&["RANGE", "COUNT"], rows);
            println!("{}", table.format());
        }
        Command::Ping => {
            client.ping().await?;
            println!("{}", PONG.format());
        }
        Command::Delete { key } => {
            client.delete(key).await?;
            println!("{}", OK.format());
        }
    };
    Ok(())
}
