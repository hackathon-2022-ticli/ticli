use clap::{Parser, Subcommand, ValueEnum, ValueHint};

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

    /// Print help information.
    // FIXME: https://github.com/clap-rs/clap/issues/4367
    // #[arg(long, action = ArgAction::Help)]
    // pub help: bool,

    /// Sub command.
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Get the value of key.
    Get { key: String },

    /// Set key to hold the string value.
    Set { key: String, value: String },
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Mode {
    Txn,
    Raw,
}
