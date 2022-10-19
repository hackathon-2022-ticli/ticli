mod cli;

use clap::Parser;
use cli::TiCLI;
use owo_colors::OwoColorize;
use std::{io, process, str};
use tikv_client::TransactionClient;

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

    let (host, port) = (ticli.host, ticli.port);
    let client = match ticli.mode {
        cli::Mode::Txn => TransactionClient::new(vec![format!("{host}:{port}")]).await?,
        cli::Mode::Raw => todo!(),
    };

    match ticli.command {
        cli::Command::Get { key } => {
            let mut txn = client.begin_optimistic().await?;
            let value = txn.get(key).await?;
            txn.commit().await?;
            match value {
                Some(buf) => println!("{}", str::from_utf8(&buf)?),
                None => println!("{}", "(nil)".bright_black().italic()),
            }
        }
        cli::Command::Set { key, value } => {
            let mut txn = client.begin_optimistic().await?;
            txn.put(key, value).await?;
            txn.commit().await?;
            println!("OK");
        }
    }
    Ok(())
}
