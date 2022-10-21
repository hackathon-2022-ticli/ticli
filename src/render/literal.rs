use owo_colors::OwoColorize;

use super::{is_tty, Render};

#[allow(clippy::upper_case_acronyms)]
pub enum Literal {
    OK,
    NIL,
    PONG,
    Goodbye,
}

impl Render for Literal {
    fn render(&self) -> String {
        match is_tty() {
            true => match self {
                Literal::OK => "OK".bright_green().to_string(),
                Literal::NIL => "(nil)".bright_black().italic().to_string(),
                Literal::PONG => "PONG".bright_green().to_string(),
                Literal::Goodbye => "Goodbye!".bright_black().to_string(),
            },
            false => match self {
                Literal::OK => "OK".into(),
                Literal::NIL => "(nil)".into(),
                Literal::PONG => "PONG".into(),
                Literal::Goodbye => "Goodbye!".into(),
            },
        }
    }
}
