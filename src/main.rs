mod cli;
mod executor;
mod model;
mod parser;
mod render;
mod repl;
mod tikv;

use anyhow::Result;
use clap::Parser;
use cli::{TiCLI, TABLE_STYLE};
use executor::execute;
use owo_colors::OwoColorize;
use render::{Literal::ERROR, Render};
use repl::Repl;
use std::{io, process};
use tikv::Client;

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                process::exit(0);
            }
        }

        eprintln!("{} {}", ERROR.render(), e);
        std::process::exit(1)
    }
}

async fn try_main() -> Result<()> {
    env_logger::init();

    let ticli = TiCLI::parse();

    {
        *TABLE_STYLE.lock().unwrap() = ticli.style;
    }

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
            let history_file = repl::history_file_from_env()?;
            if let Some(parent) = history_file.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let repl = Repl::new(client, prompt, history_file);
            repl.start().await?;
        }
        Some(cmd) => execute(&client, cmd).await?,
    }

    Ok(())
}
