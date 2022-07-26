use std::process::Command;

use crate::swaybar_object::SwaybarObject;
use regex::Regex;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    Regex(regex::Error),
    ParseInt(std::num::ParseIntError),
    IO(std::io::Error),
    FromUTF8(std::string::FromUtf8Error),
}

impl From<String> for Error {
    fn from(string: String) -> Self {
        Error::Generic(string)
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::Regex(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::FromUTF8(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(s) => f.write_str(s),
            Error::Regex(e) => e.fmt(f),
            Error::ParseInt(e) => e.fmt(f),
            Error::IO(e) => e.fmt(f),
            Error::FromUTF8(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Generic(_) => None,
            Error::Regex(e) => e.source(),
            Error::ParseInt(e) => e.source(),
            Error::IO(e) => e.source(),
            Error::FromUTF8(e) => e.source(),
        }
    }
}

#[derive(Debug)]
pub struct BattInfo {
    regex: Regex,
    acpi_error: bool,
}

impl Default for BattInfo {
    fn default() -> Self {
        Self {
            regex: Regex::new("([0-9]+)%.*").expect("Should be able to compile regex"),
            acpi_error: false,
        }
    }
}

impl BattInfo {
    pub fn is_error_state(&self) -> bool {
        self.acpi_error
    }

    pub fn update(&mut self, object: &mut SwaybarObject) -> Result<(), Error> {
        if self.acpi_error {
            return Err(Error::Generic("battinfo: in error state".into()));
        }
        let output_string: String;
        let output_percentage: u8;
        let string_result = self.get_acpi_string();
        if let Ok(string) = string_result {
            (output_string, output_percentage) = string;

            let percentage: f32 = output_percentage as f32 / 100.0f32;
            let red: u8 = if percentage > 0.5f32 {
                (255.0f32 * (1.0f32 - (percentage - 0.5f32) * 2.0f32)) as u8
            } else {
                255u8
            };
            let green: u8 = if percentage > 0.5f32 {
                255u8
            } else {
                (255.0f32 * percentage * 2.0f32) as u8
            };
            let color: String = format!("#{:x}{:x}00ff", red, green);

            object.update_as_generic(output_string, Some(color));

            Ok(())
        } else {
            self.acpi_error = true;
            string_result.map(|_| ())
        }
    }

    fn get_acpi_string(&mut self) -> Result<(String, u8), Error> {
        if self.acpi_error {
            return Err(Error::Generic("battinfo: acpi_error is true".into()));
        }
        let mut cmd_builder = Command::new("acpi");
        cmd_builder.arg("-b");
        let output = cmd_builder.output()?;
        let string = String::from_utf8(output.stdout)?;
        let regex_captures_result = self.regex.captures(&string);
        if regex_captures_result.is_none() {
            self.acpi_error = true;
            return Err(Error::Generic("battinfo: regex captured nothing".into()));
        }
        let regex_captures = regex_captures_result.unwrap();
        let full_result = regex_captures.get(0);
        if full_result.is_none() {
            self.acpi_error = true;
            return Err(Error::Generic("battinfo: no full regex capture".into()));
        }
        let full_string = full_result.unwrap().as_str().to_owned();
        let percentage_result = regex_captures.get(1);
        if percentage_result.is_none() {
            self.acpi_error = true;
            return Err(Error::Generic("battinfo: no regex capture 1".into()));
        }
        let percentage: u8 = percentage_result.unwrap().as_str().parse::<u8>()?;

        Ok((full_string, percentage))
    }
}
