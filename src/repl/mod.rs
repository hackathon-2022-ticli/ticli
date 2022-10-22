mod readline;

use std::{env, path::PathBuf};

use self::readline::{CommandCompleter, CompleteHintHandler, ReplHelper};

use crate::{
    cli::{print_repl_help, Command, TiCLI, APP_NAME},
    executor::execute,
    render::{Literal::*, Render},
    tikv::Client,
};

use anyhow::Result;
use clap::{ErrorKind, Parser};
use rustyline::{
    error::ReadlineError,
    hint::HistoryHinter,
    CompletionType,
    Config,
    EditMode,
    Editor,
    EventHandler,
    KeyEvent,
};

const HIST_FILE_NAME: &str = "history";
const HIST_PATH_ENV: &str = "TICLI_HISTFILE";

pub struct Repl {
    client:       Client,
    prompt:       String,
    history_file: PathBuf,
}

impl Repl {
    pub fn new(client: Client, prompt: impl Into<String>, history_file: PathBuf) -> Self {
        Self { client, prompt: prompt.into(), history_file }
    }

    pub async fn start(&self) -> Result<()> {
        let mut rl = Self::init_rl(&self.prompt)?;

        match rl.load_history(&self.history_file) {
            Ok(_) => {}
            Err(ReadlineError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => eprintln!("{} Failed loading history: {:?}", WARN.render(), e),
        }

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
                                    rl.append_history(&self.history_file)?;
                                    execute(&self.client, command).await?;
                                }
                                Ok(TiCLI { command: None, .. }) => {
                                    print_repl_help()?;
                                }
                                // hacking clap error to show simplified help message when possible. is there a better way?
                                Err(e) =>
                                    if e.kind() == ErrorKind::DisplayHelp && format!("{}", e).starts_with(APP_NAME) {
                                        print_repl_help()?;
                                    } else {
                                        e.print()?
                                    },
                            }
                        }
                        None => {
                            eprintln!("{} invalid quoting", ERROR.render());
                        }
                    }
                }
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    execute(&self.client, Command::Quit).await?;
                }
                Err(err) => {
                    eprintln!("{} {:?}", ERROR.render(), err);
                    break;
                }
            }
        }
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
        };
        let handler = Box::new(CompleteHintHandler);

        let mut rl = Editor::with_config(config)?;
        rl.bind_sequence(KeyEvent::ctrl('E'), EventHandler::Conditional(handler.clone()));
        rl.bind_sequence(KeyEvent::alt('f'), EventHandler::Conditional(handler));
        rl.set_helper(Some(helper));

        Ok(rl)
    }
}

pub fn history_file_from_env() -> Result<PathBuf> {
    match env::var(HIST_PATH_ENV) {
        Ok(path) => Ok(path.into()),
        Err(_) => Ok(xdg::BaseDirectories::with_prefix(APP_NAME)?.get_state_file(HIST_FILE_NAME)),
    }
}
