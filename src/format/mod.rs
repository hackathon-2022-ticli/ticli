use owo_colors::OwoColorize;

pub mod table;

#[allow(clippy::upper_case_acronyms)]
pub enum Literal {
    OK,
    NIL,
    PONG,
}

impl Literal {
    pub fn print(&self) {
        match self {
            Literal::OK => println!("{}", "OK".bright_green()),
            Literal::NIL => println!("{}", "(nil)".bright_black().italic()),
            Literal::PONG => println!("{}", "PONG".bright_green()),
        }
    }
}
