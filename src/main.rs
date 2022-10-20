mod cli;
mod client;
mod format;
mod repl;
mod runner;

use anyhow::Result;
use clap::Parser;
use cli::TiCLI;
use client::Client;
use owo_colors::OwoColorize;
use repl::Repl;
use runner::run_cmd;
use std::{io, process};

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

async fn try_main() -> Result<()> {
    env_logger::init();

    let ticli = TiCLI::parse();

    let client = match ticli.mode {
        cli::Mode::Txn => Client::txn(ticli.addr()).await?,
        cli::Mode::Raw => Client::raw(ticli.addr()).await?,
    };

    match ticli.command {
        None => {
            let mode = match ticli.mode {
                cli::Mode::Txn => "TiKV@Txn".blue().bold().to_string(),
                cli::Mode::Raw => "TiKV@Raw".yellow().bold().to_string(),
            };
            let prompt = format!("{} {}> ", mode, ticli.addr());
            let repl = Repl::new(client, prompt);
            repl.start().await?;
        }
        Some(cmd) => run_cmd(&client, cmd).await?,
    }

    Ok(())
}
