use std::{
    fs::File,
    io::{self, Read},
};

use anyhow::Result;
use async_recursion::async_recursion;
use tikv_client::{BoundRange, KvPair};
use tokio::time::Instant;

use crate::{
    cli::Command,
    format::{DurationExt, Literal::*, Table},
    parser,
    tikv::{BoundRangeExt, Client},
};

macro_rules! time_it {
    ($code:block) => {{
        let start = Instant::now();
        $code;
        println!("{}", start.elapsed().format());
    }};
}

#[async_recursion(?Send)]
pub async fn execute(client: &Client, cmd: Command) -> Result<()> {
    match cmd {
        Command::Get { key } => {
            time_it! {{
                let value = client.get(key.clone()).await?;
                let kv_pair = value.map(|value| KvPair::new(key, value));
                let table: Table = kv_pair.into();
                println!("{}", table.format());
            }}
        }
        Command::Set { key, value } => {
            time_it! {{
                client.set(key, value).await?;
                println!("{}", OK.format());
            }}
        }
        Command::Scan { from, to, limit } => {
            time_it! {{
                let range: BoundRange = BoundRangeExt::build(from, to);
                let kvs = client.scan(range, limit).await?;
                let table: Table = kvs.into();
                println!("{}", table.with_seq().format());
            }}
        }
        Command::Count { from, to } => {
            time_it! {{
                let range: BoundRange = BoundRangeExt::build(from, to);
                let count = client.count(range.clone()).await?;
                let rows = vec![vec![range.to_string(), count.to_string()]];
                let table = Table::new(&["RANGE", "COUNT"], rows);
                println!("{}", table.format());
            }}
        }
        Command::Delete { key } => {
            time_it! {{
                client.delete(key).await?;
                println!("{}", OK.format());
            }}
        }
        Command::Source { file } => {
            time_it! {{
                let file: Box<dyn Read> = match file {
                    Some(file) => Box::new(File::open(file)?),
                    None => Box::new(io::stdin()),
                };
                for cmd in parser::from_reader(file) {
                    execute(client, cmd?).await?;
                }
            }}
        }
        Command::Strlen { key } => {
            time_it! {{
                let len = client.strlen(key.clone()).await?;
                let row = len.map(|len| vec![vec![key, len.to_string()]]);
                let table = Table::new(&["KEY", "LENGTH"], row.unwrap_or_default());
                println!("{}", table.format());
            }}
        }
        Command::Ping => {
            time_it! {{
                client.ping().await?;
                println!("{}", PONG.format());
            }}
        }
        Command::Quit => {
            println!("{}", Goodbye.format());
            std::process::exit(0);
        }
        Command::Noop => {}
    };
    Ok(())
}
