use owo_colors::OwoColorize;

pub mod duration;
pub mod table;

#[allow(clippy::upper_case_acronyms)]
pub enum Literal {
    OK,
    NIL,
    PONG,
    Goodbye,
}

impl Literal {
    pub fn format(&self) -> String {
        match self {
            Literal::OK => "OK".bright_green().to_string(),
            Literal::NIL => "(nil)".bright_black().italic().to_string(),
            Literal::PONG => "PONG".bright_green().to_string(),
            Literal::Goodbye => "Goodbye!".bright_black().to_string(),
        }
    }
}
