use std::process::Command;

use crate::swaybar_object::SwaybarObject;
use regex::Regex;

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

    pub fn update(&mut self, object: &mut SwaybarObject) -> Result<(), ()> {
        if self.acpi_error {
            return Err(());
        }
        let output_string: String;
        let output_percentage: u8;
        if let Ok(string) = self.get_acpi_string() {
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
            Err(())
        }
    }

    fn get_acpi_string(&mut self) -> Result<(String, u8), ()> {
        if self.acpi_error {
            return Err(());
        }
        let mut cmd_builder = Command::new("acpi");
        cmd_builder.arg("-b");
        let output_result = cmd_builder.output();
        if let Ok(output_unwrapped) = output_result {
            let string_result = String::from_utf8(output_unwrapped.stdout);
            if let Ok(string) = string_result {
                let regex_captures_result = self.regex.captures(&string);
                if regex_captures_result.is_none() {
                    self.acpi_error = true;
                    return Err(());
                }
                let regex_captures = regex_captures_result.unwrap();
                let full_result = regex_captures.get(0);
                if full_result.is_none() {
                    self.acpi_error = true;
                    return Err(());
                }
                let full_string = full_result.unwrap().as_str().to_owned();
                let percentage_result = regex_captures.get(1);
                if percentage_result.is_none() {
                    self.acpi_error = true;
                    return Err(());
                }
                let percentage_result = percentage_result.unwrap().as_str().parse::<u8>();
                if percentage_result.is_err() {
                    self.acpi_error = true;
                    return Err(());
                }
                let percentage: u8 = percentage_result.unwrap();

                Ok((full_string, percentage))
            } else {
                self.acpi_error = true;
                Err(())
            }
        } else {
            self.acpi_error = true;
            Err(())
        }
    }
}
