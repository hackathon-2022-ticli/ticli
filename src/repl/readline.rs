use anyhow::Result;
use lazy_static::lazy_static;
use owo_colors::OwoColorize;
use regex::Regex;
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::{
    error::ReadlineError,
    highlight::Highlighter,
    hint::HistoryHinter,
    Cmd,
    ConditionalEventHandler,
    Event,
    EventContext,
    KeyEvent,
    RepeatCount,
};

use crate::cli::COMMAND_VARIANTS;

lazy_static! {
    static ref RE_COMMANDS: Regex = Regex::new(&format!(r"(?i)^\s*({})\b", COMMAND_VARIANTS.join("|"))).unwrap();
    static ref RE_FIRST_WORD: Regex = Regex::new(r"(?i)^\s*\S+").unwrap();
}

pub(super) struct CommandCompleter;

impl rustyline::completion::Completer for CommandCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<String>), ReadlineError> {
        let mut completions = Vec::new();
        let mut line = line[..pos].trim_end();
        if line.is_empty() {
            return Ok((0, completions));
        }
        if line.starts_with(':') {
            line = &line[1..];
        }
        if line.is_empty() {
            return Ok((0, completions));
        }
        for cmd in COMMAND_VARIANTS.iter() {
            if cmd.to_lowercase().starts_with(line) {
                completions.push(cmd.to_lowercase());
            } else if cmd.to_uppercase().starts_with(line) {
                completions.push(cmd.to_uppercase());
            }
        }
        Ok((0, completions))
    }
}

#[derive(Helper, Completer, Hinter, Validator)]
pub(super) struct ReplHelper {
    #[rustyline(Completer)]
    pub(super) completer: CommandCompleter,

    #[rustyline(Hinter)]
    pub(super) hinter: HistoryHinter,

    pub(super) colored_prompt: String,
}

impl Highlighter for ReplHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, default: bool) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(hint.bright_black().to_string())
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        RE_FIRST_WORD.replace(line, |cap: &regex::Captures| {
            let word = &cap[0];
            match RE_COMMANDS.is_match(word) {
                true => word.to_uppercase().bright_green().to_string(),
                false => word.bright_red().to_string(),
            }
        })
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        true
    }
}

#[derive(Clone)]
pub(super) struct CompleteHintHandler;

impl ConditionalEventHandler for CompleteHintHandler {
    fn handle(&self, evt: &Event, _: RepeatCount, _: bool, ctx: &EventContext) -> Option<Cmd> {
        let k = ctx.has_hint().then_some(evt.get(0))??;
        if *k == KeyEvent::ctrl('E') {
            Some(Cmd::CompleteHint)
        } else if *k == KeyEvent::alt('f') && ctx.line().len() == ctx.pos() {
            let text = ctx.hint_text()?;
            let start = match text.chars().next() {
                Some(fst) if !fst.is_alphanumeric() => text.find(|c: char| c.is_alphanumeric()).unwrap_or_default(),
                _ => 0,
            };
            let text = text
                .chars()
                .enumerate()
                .take_while(|(i, c)| *i <= start || c.is_alphanumeric())
                .map(|(_, c)| c)
                .collect::<String>();
            Some(Cmd::Insert(1, text))
        } else {
            None
        }
    }
}
