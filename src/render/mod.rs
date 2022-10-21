mod literal;
mod table;

use std::time::Duration;

pub use literal::Literal;
pub use table::Table;

use owo_colors::OwoColorize;
use tikv_client::{Key, Value};

pub trait Render {
    fn render(&self) -> String;

    fn print(&self) {
        println!("{}", self.render())
    }
}

pub fn string_or_base64(buf: &[u8]) -> String {
    match std::str::from_utf8(buf) {
        Ok(s) => s.to_string(),
        Err(_) => match is_tty() {
            true => format!("{}", base64::encode(buf).purple()),
            false => base64::encode(buf),
        },
    }
}

pub fn is_tty() -> bool {
    atty::is(atty::Stream::Stdout)
}

impl Render for Key {
    fn render(&self) -> String {
        string_or_base64(self.into())
    }
}

impl Render for Value {
    fn render(&self) -> String {
        string_or_base64(self)
    }
}

impl Render for Duration {
    fn render(&self) -> String {
        format!("Time: {:.03}s", self.as_secs_f32())
    }

    fn print(&self) {
        if is_tty() {
            println!("{}", self.render())
        }
    }
}

macro_rules! impl_renderer_for {
    ($($t:ty),*) => {
        $(
            impl Render for $t {
                fn render(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_renderer_for!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, String, &str);
