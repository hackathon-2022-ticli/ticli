use std::{
    fs::File,
    io::{self, Read},
};

use anyhow::Result;
use async_recursion::async_recursion;
use tikv_client::BoundRange;
use tokio::time::Instant;

use crate::{
    cli::Command,
    model::{KVResult, ScanResult},
    parser,
    render::{Literal::*, Render},
    tikv::{BoundRangeExt, Client},
};

macro_rules! time_it {
    ($code:block) => {{
        let start = Instant::now();
        $code;
        start.elapsed().print();
    }};
}

#[async_recursion(?Send)]
pub async fn execute(client: &Client, cmd: Command) -> Result<()> {
    match cmd {
        Command::Get { key } => {
            time_it! {{
                let value = client.get(key.clone()).await?;
                let res = KVResult::from_get(key, value);
                res.print();
            }}
        }
        Command::Set { key, value } => {
            time_it! {{
                client.set(key, value).await?;
                OK.print();
            }}
        }
        Command::Scan { from, to, limit } => {
            time_it! {{
                let range: BoundRange = BoundRangeExt::build(from, to);
                let res: ScanResult  = client.scan(range, limit).await?.into();
                res.print();
            }}
        }
        Command::Count { from, to } => {
            time_it! {{
                let range: BoundRange = BoundRangeExt::build(from, to);
                let count = client.count(range.clone()).await?;
                let res = KVResult::from_count(range, Some(count));
                res.print();
            }}
        }
        Command::Delete { key } => {
            time_it! {{
                client.delete(key).await?;
                OK.print();
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
                let res = KVResult::from_strlen(key, len);
                res.print();
            }}
        }
        Command::Ping => {
            time_it! {{
                client.ping().await?;
                PONG.print();
            }}
        }
        Command::Quit => {
            BYE.print();
            std::process::exit(0);
        }
        Command::Noop => {}
    };
    Ok(())
}
