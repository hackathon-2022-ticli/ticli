use clap::{ArgAction, Parser, Subcommand, ValueEnum, ValueHint};

#[derive(Parser)]
#[command(about, version)]
#[command(disable_help_flag = true)]
#[command(next_line_help = true)]
pub struct TiCLI {
    /// TiKV PD server hostname.
    #[arg(short = 'h', long, default_value = "localhost", value_hint = ValueHint::Hostname)]
    pub host: String,

    /// TiKV PD server port.
    #[arg(short, long, default_value_t = 2379)]
    pub port: u16,

    /// TiKV API mode.
    #[arg(short, long, value_enum, default_value_t = Mode::Txn)]
    pub mode: Mode,

    /// Sub command.
    #[command(subcommand)]
    pub command: Command,

    /// Print help information.
    #[arg(long, action = ArgAction::Help, value_parser = clap::value_parser!(bool))]
    pub help: (),
}

#[derive(Subcommand)]
pub enum Command {
    /// Get the value of key.
    Get { key: String },

    /// Set key to hold the string value.
    Set { key: String, value: String },

    /// Scan keys from the start prefix.
    Scan {
        /// Key prefix.
        prefix: String,

        /// Limit the number of records to scan.
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },

    /// Return pong when connection is alive.
    Ping {},
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Mode {
    Txn,
    Raw,
}
