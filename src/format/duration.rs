use std::time::Duration;

pub trait DurationExt {
    fn format(&self) -> String;
}

impl DurationExt for Duration {
    fn format(&self) -> String {
        format!("Time: {:.03}s", self.as_secs_f32())
    }
}
