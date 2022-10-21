use clap::{ArgAction, Parser, Subcommand, ValueEnum, ValueHint};

#[derive(Debug, Parser)]
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
    pub command: Option<Command>,

    /// Print help information.
    #[arg(long, action = ArgAction::Help, value_parser = clap::value_parser!(bool))]
    pub help: (),
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Get the value of key.
    #[command(visible_aliases = ["GET"])]
    Get { key: String },

    /// Set key to hold the string value.
    #[command(visible_aliases = ["SET"])]
    Set { key: String, value: String },

    /// Delete the specified key.
    #[command(visible_aliases = ["DELETE", "del", "DEL"])]
    Delete { key: String },

    /// Scan keys between the range.
    #[command(visible_aliases = ["SCAN"])]
    Scan {
        /// Start key.
        #[arg(long)]
        from: Option<String>,

        /// End Key (included).
        #[arg(long)]
        to: Option<String>,

        /// Limit the number of records to scan.
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },

    /// Count keys between the range.
    #[command(visible_aliases = ["COUNT", "cnt", "CNT"])]
    Count {
        /// Start key.
        #[arg(long)]
        from: Option<String>,

        /// End Key (included).
        #[arg(long)]
        to: Option<String>,
    },

    /// Return pong when connection is alive.
    #[command(visible_aliases = ["PING"])]
    Ping,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Mode {
    Txn,
    Raw,
}

impl TiCLI {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
