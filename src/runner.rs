use anyhow::Result;
use tikv_client::KvPair;

use crate::{
    cli::Command,
    client::Client,
    format::{table::Table, Literal::*},
};

pub async fn run_cmd(client: &Client, cmd: Command) -> Result<()> {
    match cmd {
        Command::Get { key } => {
            let value = client.get(key.clone()).await?;
            let kv_pair = value.map(|value| KvPair::new(key, value));
            let table: Table = kv_pair.into();
            table.print();
        }
        Command::Set { key, value } => {
            client.set(key, value).await?;
            OK.print();
        }
        Command::Scan { from, to, limit } => {
            let kvs = client.scan(from, to, limit).await?;
            let table: Table = kvs.into();
            table.with_seq().print();
        }
        Command::Ping => {
            client.ping().await?;
            PONG.print();
        }
        Command::Delete { key } => {
            client.delete(key).await?;
            OK.print();
        }
    };
    Ok(())
}
