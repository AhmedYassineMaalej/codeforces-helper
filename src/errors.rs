use std::{fmt::Display, io};

#[derive(Debug)]
pub enum CliError {
    Io(String, io::Error),
    Compilation,
    Execution(String),
    Clipboard(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Io(desc, error) => write!(f, "IO error: {desc}\n{error}"),
            CliError::Compilation => write!(f, "compilation error: Compilation failed"),
            CliError::Execution(e) => write!(f, "execution error: {e}"),
            CliError::Clipboard(e) => write!(f, "clipboard error: {e}"),
        }
    }
}
