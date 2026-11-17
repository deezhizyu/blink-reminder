use std::fmt::Display;

pub fn report_error(context: &str, error: &impl Display) {
    eprintln!("{context}: {error}");
}
