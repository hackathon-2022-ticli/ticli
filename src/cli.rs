use clap::{AppSettings, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use lazy_static::lazy_static;
use std::{path::PathBuf, sync::Mutex};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

lazy_static! {
    pub static ref TABLE_STYLE: Mutex<TableStyle> = Mutex::new(TableStyle::default());
}

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

    /// Specify the output table style.
    #[clap(short, long, value_enum, default_value_t = TableStyle::default())]
    pub style: TableStyle,

    /// Sub command.
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Get the value of key.
    #[clap(aliases = &["GET"])]
    Get { key: String },

    /// Get the value of key in binary format.
    #[clap(aliases = &["GETB"])]
    Getb { key: String },

    /// Set key to hold the string value.
    #[clap(aliases = &["SET"])]
    Set {
        key:   String,
        /// Value string to set.
        value: String,
    },

    /// Set key to hold the binary data from the file.
    #[clap(aliases = &["SETB"])]
    Setb {
        key:  String,
        /// Binary file to set (ignore to read from standard input).
        #[clap(name = "FILE", value_hint = ValueHint::FilePath)]
        file: Option<PathBuf>,
    },

    /// Increments the number stored at key by one.
    #[clap(aliases = &["INCR"])]
    Incr { key: String },

    /// Increments the number stored at key by increment.
    #[clap(aliases = &["INCRBY"])]
    Incrby { key: String, increment: i128 },

    /// Decrements the number stored at key by one.
    #[clap(aliases = &["DECR"])]
    Decr { key: String },

    /// Decrements the number stored at key by decrement.
    #[clap(aliases = &["DECRBY"])]
    Decrby { key: String, decrement: i128 },

    /// Delete the specified key.
    #[clap(visible_aliases = &["del"], aliases = &["DELETE", "DEL"])]
    Delete { key: String },

    /// Get the length of the bytes stored at key.
    #[clap(aliases = &["STRLEN"])]
    Strlen { key: String },

    /// Returns if key exists.
    #[clap(aliases = &["EXISTS"])]
    Exists { key: String },

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
    #[clap(aliases = &["SOURCE"])]
    Source {
        /// File to source (ignore to read from standard input).
        #[clap(name = "FILE", value_hint = ValueHint::FilePath)]
        file: Option<PathBuf>,
    },

    /// Load kv records from csv file.
    #[clap(aliases = &["LOADCSV"])]
    Loadcsv {
        /// File to load (ignore to read from standard input).
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

    /// Remove all keys from tikv.
    #[clap(aliases = &["FLUSHALL"])]
    Flushall,

    /// Return pong when connection is alive.
    #[clap(aliases = &["PING"])]
    Ping,

    /// Specify the output table style.
    #[clap(aliases = &["STYLE"])]
    Style {
        #[clap(value_enum)]
        style: Option<TableStyle>,
    },

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

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum TableStyle {
    #[default]
    Modern,
    Sharp,
    Rounded,
    Bare,
    Ascii,
    Psql,
    Text,
    Markdown,
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
        .next_line_help(false)
        .override_usage("COMMAND [OPTIONS]")
        .subcommand_help_heading("COMMANDS")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .print_help()
}

lazy_static! {
    pub static ref COMMAND_VARIANTS: Vec<String> = {
        TiCLI::command()
            .get_subcommands()
            .flat_map(|cmd| cmd.get_all_aliases().chain([cmd.get_name(), "help"]))
            .map(|s| s.to_string())
            .collect()
    };
}
