use regex::Regex;
use std::io;
use std::process::Command;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    FromUTF8(std::string::FromUtf8Error),
    Generic(String),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self::IO(io_error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(utf8_error: std::string::FromUtf8Error) -> Self {
        Self::FromUTF8(utf8_error)
    }
}

impl From<String> for Error {
    fn from(string: String) -> Self {
        Self::Generic(string)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => e.fmt(f),
            Error::FromUTF8(e) => e.fmt(f),
            Error::Generic(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(e) => e.source(),
            Error::FromUTF8(e) => e.source(),
            Error::Generic(_) => None,
        }
    }
}

pub fn get_cmd_output(cmd: &str, args: &[&str], regex: &Regex) -> Result<String, Error> {
    let mut cmd_builder = Command::new(cmd);
    for arg in args {
        cmd_builder.arg(arg);
    }
    let output = cmd_builder.output()?;
    let stdout_output: String = String::from_utf8(output.stdout)?;
    let regex_captures = regex
        .captures(&stdout_output)
        .ok_or_else(|| Error::from("Regex returned empty matches".to_owned()))?;
    let regex_match = regex_captures
        .get(0)
        .ok_or_else(|| Error::from("Failed to get regex match".to_owned()))?;
    Ok(regex_match.as_str().to_owned())
}
