use anyhow::Result;
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::{
    error::ReadlineError,
    highlight::{Highlighter, MatchingBracketHighlighter},
    hint::HistoryHinter,
    validate::MatchingBracketValidator,
    Cmd,
    ConditionalEventHandler,
    Event,
    EventContext,
    KeyEvent,
    RepeatCount,
};

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
        // TODO: get commands list directly from Command enum
        for cmd in &["quit", "exit", "help", "get", "set", "scan"] {
            if cmd.starts_with(line) {
                completions.push(cmd.to_string());
            }
        }
        Ok((0, completions))
    }
}

#[derive(Helper, Completer, Hinter, Validator)]
pub(super) struct ReplHelper {
    #[rustyline(Completer)]
    pub(super) completer: CommandCompleter,

    pub(super) highlighter: MatchingBracketHighlighter,

    #[rustyline(Validator)]
    pub(super) validator: MatchingBracketValidator,

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
        Owned("\x1b[1;30m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
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
