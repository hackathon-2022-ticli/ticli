use owo_colors::OwoColorize;

use super::{is_tty, Render};

#[allow(clippy::upper_case_acronyms)]
pub enum Literal {
    OK,
    NIL,
    PONG,
    BYE,
    ERROR,
    WARN,
}

impl Render for Literal {
    fn render(&self) -> String {
        match is_tty() {
            true => match self {
                Literal::OK => "OK".bright_green().to_string(),
                Literal::NIL => "(nil)".bright_black().italic().to_string(),
                Literal::PONG => "PONG".bright_green().to_string(),
                Literal::BYE => "Goodbye!".bright_black().to_string(),
                Literal::ERROR => "error:".bright_red().bold().to_string(),
                Literal::WARN => "warn:".bright_yellow().bold().to_string(),
            },
            false => match self {
                Literal::OK => "OK".into(),
                Literal::NIL => "(nil)".into(),
                Literal::PONG => "PONG".into(),
                Literal::BYE => "Goodbye!".into(),
                Literal::ERROR => "error:".into(),
                Literal::WARN => "warn:".into(),
            },
        }
    }
}
