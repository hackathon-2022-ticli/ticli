use clap::{builder::StyledStr, ArgAction, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use owo_colors::OwoColorize;
use std::path::PathBuf;
use strum::EnumVariantNames;

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

#[derive(Debug, Subcommand, EnumVariantNames)]
pub enum Command {
    /// Get the value of key.
    #[command(aliases = ["GET"])]
    Get { key: String },

    /// Set key to hold the string value.
    #[command(aliases = ["SET"])]
    Set { key: String, value: String },

    /// Delete the specified key.
    #[command(visible_aliases = ["del"], aliases = ["DELETE", "DEL"])]
    Delete { key: String },

    /// Get the length of the bytes stored at key.
    #[command(aliases = ["STRLEN"])]
    Strlen { key: String },

    /// Scan keys between the range.
    #[command(aliases = ["SCAN"])]
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
    #[command(visible_aliases = ["cnt"], aliases = ["COUNT", "CNT"])]
    Count {
        /// Start key.
        #[arg(long)]
        from: Option<String>,

        /// End Key (included).
        #[arg(long)]
        to: Option<String>,
    },

    /// Execute commands from file.
    #[command(visible_aliases = ["."], aliases = ["SOURCE"])]
    Source {
        /// File to source. Ignore to read from standard input.
        #[arg(name = "FILE", value_hint = ValueHint::FilePath)]
        file: Option<PathBuf>,
    },

    /// Return pong when connection is alive.
    #[command(aliases = ["PING"])]
    Ping,

    /// Exit the program.
    #[command(visible_aliases = ["exit"], aliases = ["QUIT", "EXIT"])]
    Quit,

    /// No Operation.
    #[command(hide = true)]
    Noop,
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

pub fn render_repl_help() -> StyledStr {
    let mut cmd = TiCLI::command();
    for arg in TiCLI::command().get_arguments() {
        cmd = cmd.mut_arg(arg.get_id(), |a| a.hide(true));
    }
    cmd.disable_version_flag(true)
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .help_template(format!("{}\n{{subcommands}}", "\nCOMMANDS:".bold().underline()))
        .render_help()
}
