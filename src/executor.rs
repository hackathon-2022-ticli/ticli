use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use anyhow::Result;
use async_recursion::async_recursion;
use tikv_client::BoundRange;
use tokio::{
    io::{stdout, AsyncWriteExt},
    time::Instant,
};

use crate::{
    cli::{Command, TABLE_STYLE},
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
                let res = KVResult::from_kv(key, value);
                res.print();
            }}
        }
        Command::Getb { key } => {
            time_it! {{
                if let Some(value) = client.get(key.clone()).await? {
                    stdout().write_all(&value).await?;
                }
            }}
        }
        Command::Set { key, value } => {
            time_it! {{
                client.set(key, value).await?;
                OK.print();
            }}
        }
        Command::Setb { key, file } => {
            time_it! {{
                let mut rdr = create_reader(file)?;
                let mut buf = Vec::new();
                rdr.read_to_end(&mut buf)?;
                client.set(key, buf).await?;
                OK.print();
            }}
        }
        Command::Incrby { key, increment } => {
            time_it! {{
                let value = client.incr_by(key.clone(), increment).await?;
                let res = KVResult::from_kv(key, value);
                res.print();
            }}
        }
        Command::Incr { key } => execute(client, Command::Incrby { key, increment: 1 }).await?,
        Command::Decrby { key, decrement } => execute(client, Command::Incrby { key, increment: -decrement }).await?,
        Command::Decr { key } => execute(client, Command::Decrby { key, decrement: 1 }).await?,
        Command::Scan { from, to, limit, output } => {
            time_it! {{
                let range: BoundRange = BoundRangeExt::build(from, to);
                let res: ScanResult  = client.scan(range, limit).await?.into();
                res.print_with_format(output);
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
        Command::Flushall => {
            time_it! {{
                client.flush_all().await?;
                OK.print();
            }}
        }
        Command::Source { file } => {
            time_it! {{
                let rdr = create_reader(file)?;
                for cmd in parser::from_reader(rdr) {
                    execute(client, cmd?).await?;
                }
            }}
        }
        Command::Loadcsv { file, header, delimiter, batch } => {
            time_it! {{
                let rdr = create_reader(file)?;
                client.load_csv(rdr, header, delimiter, batch).await?;
            }}
        }
        Command::Strlen { key } => {
            time_it! {{
                let len = client.strlen(key.clone()).await?;
                let res = KVResult::from_strlen(key, len);
                res.print();
            }}
        }
        Command::Exists { key } => {
            time_it! {{
                let res = client.exists(key).await?;
                res.print();
            }}
        }
        Command::Ping => {
            time_it! {{
                client.ping().await?;
                PONG.print();
            }}
        }
        Command::Style { style } => {
            time_it! {{
                if let Some(style) = style {
                    *TABLE_STYLE.lock().unwrap() = style;
                }
                (*TABLE_STYLE.lock().unwrap()).print()
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

fn create_reader(file: Option<PathBuf>) -> Result<Box<dyn Read>> {
    match file {
        Some(file) => Ok(Box::new(File::open(file)?)),
        None => Ok(Box::new(io::stdin())),
    }
}
