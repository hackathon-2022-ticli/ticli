use clap::{AppSettings, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use std::path::PathBuf;
use strum::EnumVariantNames;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(next_line_help = true)]

pub struct TiCLI {
    /// TiKV PD server hostname.
    #[clap(short = 'h', long, default_value = "127.0.0.1", value_hint = ValueHint::Hostname)]
    pub host: String,

    /// TiKV PD server port.
    #[clap(short, long, default_value_t = 2379)]
    pub port: u16,

    /// TiKV API mode.
    #[clap(short, long, value_enum, default_value_t = Mode::Txn)]
    pub mode: Mode,

    /// Sub command.
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand, EnumVariantNames)]
pub enum Command {
    /// Get the value of key.
    #[clap(aliases = &["GET"])]
    Get { key: String },

    /// Get the value of key in binary format.
    #[clap(aliases = &["GETB"])]
    Getb { key: String },

    /// Set key to hold the string value.
    #[clap(aliases = &["SET"])]
    Set { key: String, value: String },

    /// Set key to hold the binary value from a file.
    #[clap(name = "setb", aliases = &["SETB"])]
    SetB {
        key:  String,
        /// Binary file to set. Ignore to read from standard input.
        #[clap(name = "FILE", value_hint = ValueHint::FilePath)]
        file: Option<PathBuf>,
    },

    /// Delete the specified key.
    #[clap(visible_aliases = &["del"], aliases = &["DELETE", "DEL"])]
    Delete { key: String },

    /// Get the length of the bytes stored at key.
    #[clap(aliases = &["STRLEN"])]
    Strlen { key: String },

    /// Scan keys between the range.
    #[clap(aliases = &["SCAN"])]
    Scan {
        /// Start Key prefix (included).
        #[clap(long)]
        from: Option<String>,

        /// End Key prefix (included).
        #[clap(long)]
        to: Option<String>,

        /// Limit the number of records to scan.
        #[clap(short, long, default_value_t = 10)]
        limit: usize,

        /// Output format.
        #[clap(short, long, value_enum, default_value_t = OutputFormat::Auto)]
        output: OutputFormat,
    },

    /// Count keys between the range.
    #[clap(visible_aliases = &["cnt"], aliases = &["COUNT", "CNT"])]
    Count {
        /// Start Key prefix (included).
        #[clap(long)]
        from: Option<String>,

        /// End Key prefix (included).
        #[clap(long)]
        to: Option<String>,
    },

    /// Execute commands from file.
    #[clap(visible_aliases = &["."], aliases = &["SOURCE"])]
    Source {
        /// File to source. Ignore to read from standard input.
        #[clap(name = "FILE", value_hint = ValueHint::FilePath)]
        file: Option<PathBuf>,
    },

    /// Load kv records from csv file.
    #[clap(name = "loadcsv", aliases = &["LOADCSV"])]
    LoadCSV {
        /// File to load. Ignore to read from standard input.
        #[clap(name = "FILE", value_hint = ValueHint::FilePath)]
        file: Option<PathBuf>,

        /// Specify that the input has header row.
        #[clap(short, long)]
        header: bool,

        /// Specify the field delimiter.
        #[clap(short, long, default_value_t = ',')]
        delimiter: char,

        /// Specify how many records to write at once.
        #[clap(short, long, default_value_t = 100)]
        batch: usize,
    },

    /// Return pong when connection is alive.
    #[clap(aliases = &["PING"])]
    Ping,

    /// Exit the program.
    #[clap(visible_aliases = &["exit"], aliases = &["QUIT", "EXIT"])]
    Quit,

    /// No Operation.
    #[clap(hide = true)]
    Noop,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Mode {
    Txn,
    Raw,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Auto,
    Table,
    Json,
    Csv,
}

impl TiCLI {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn print_repl_help() -> Result<(), std::io::Error> {
    let mut cmd = TiCLI::command();
    for arg in TiCLI::command().get_arguments() {
        cmd = cmd.mut_arg(arg.get_id(), |a| a.hide(true));
    }
    cmd.about(None)
        .override_usage("COMMAND [OPTIONS]")
        .subcommand_help_heading("COMMANDS")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .print_help()
}
