mod readline;

use self::readline::{CommandCompleter, CompleteHintHandler, ReplHelper};

use crate::{
    cli::{render_repl_help, TiCLI},
    client::Client,
    executor::execute,
};

use anyhow::Result;
use clap::{error::ErrorKind, Parser};
use owo_colors::OwoColorize;
use rustyline::{
    error::ReadlineError,
    highlight::MatchingBracketHighlighter,
    hint::HistoryHinter,
    validate::MatchingBracketValidator,
    CompletionType,
    Config,
    EditMode,
    Editor,
    EventHandler,
    KeyEvent,
};

const HISTORY_FILE: &str = "/tmp/.ticli_history";

pub struct Repl {
    client: Client,
    prompt: String,
}

impl Repl {
    pub fn new(client: Client, prompt: impl Into<String>) -> Self {
        Self { client, prompt: prompt.into() }
    }

    pub async fn start(&self) -> Result<()> {
        let mut rl = Self::init_rl(&self.prompt)?;

        loop {
            let readline = rl.readline(&self.prompt);
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    rl.add_history_entry(line);

                    match shlex::split(line) {
                        Some(args) => {
                            let args = std::iter::once("".to_string()).chain(args);
                            match TiCLI::try_parse_from(args) {
                                Ok(TiCLI { command: Some(command), .. }) => {
                                    execute(&self.client, command).await?;
                                }
                                Ok(TiCLI { command: None, .. }) => {
                                    println!("{}", render_repl_help());
                                }
                                Err(e) =>
                                    if e.kind() == ErrorKind::DisplayHelp {
                                        println!("{}", render_repl_help());
                                    } else {
                                        e.print()?;
                                        println!();
                                    },
                            }
                        }
                        None => {
                            println!("{} invalid quoting", "error:".bright_red().bold());
                        }
                    }
                }
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    println!("Goodbye!");
                    break;
                }
                Err(err) => {
                    println!("{} {:?}", "error:".bright_red().bold(), err);
                    break;
                }
            }
        }
        rl.append_history(HISTORY_FILE)?;
        Ok(())
    }

    // To debug rustyline:
    // RUST_LOG=rustyline=debug cargo run ... 2> debug.log
    fn init_rl(prompt: impl Into<String>) -> Result<Editor<ReplHelper>> {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::Circular)
            .edit_mode(EditMode::Emacs)
            .build();

        let helper = ReplHelper {
            colored_prompt: prompt.into(),
            completer:      CommandCompleter,
            hinter:         HistoryHinter {},
            highlighter:    MatchingBracketHighlighter::new(),
            validator:      MatchingBracketValidator::new(),
        };
        let handler = Box::new(CompleteHintHandler);

        let mut rl = Editor::with_config(config)?;
        rl.bind_sequence(KeyEvent::ctrl('E'), EventHandler::Conditional(handler.clone()));
        rl.bind_sequence(KeyEvent::alt('f'), EventHandler::Conditional(handler));
        rl.set_helper(Some(helper));
        match rl.load_history(HISTORY_FILE) {
            Ok(_) => {}
            Err(ReadlineError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => println!("{} Failed loading history: {:?}", "warn:".bright_yellow().bold(), e),
        }

        Ok(rl)
    }
}
