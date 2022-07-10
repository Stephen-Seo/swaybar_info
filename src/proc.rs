use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
    Format(std::fmt::Error),
    Generic(String),
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self::IO(io_error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(parse_error: std::num::ParseIntError) -> Self {
        Self::ParseInt(parse_error)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(fmt_error: std::fmt::Error) -> Self {
        Self::Format(fmt_error)
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
            Error::ParseInt(e) => e.fmt(f),
            Error::Format(e) => e.fmt(f),
            Error::Generic(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(e) => e.source(),
            Error::ParseInt(e) => e.source(),
            Error::Format(e) => e.source(),
            Error::Generic(_) => None,
        }
    }
}

pub struct NetInfo {
    dev_name: String,
    down: u64,
    prev_down: u64,
    up: u64,
    prev_up: u64,
}

impl NetInfo {
    pub fn new(dev_name: String) -> Self {
        Self {
            dev_name,
            down: 0,
            prev_down: 0,
            up: 0,
            prev_up: 0,
        }
    }

    pub fn update(&mut self) -> Result<(), Error> {
        let mut netdev_string = String::new();
        {
            let mut netdev_file: File = File::open("/proc/net/dev")?;
            netdev_file.read_to_string(&mut netdev_string)?;
        }

        let mut dev_line: Option<&str> = None;
        for line in netdev_string.lines() {
            if line.starts_with(&self.dev_name) {
                dev_line = Some(line);
                break;
            }
        }

        if let Some(line) = dev_line {
            let entries: Vec<&str> = line.split_whitespace().collect();
            if entries.len() < 10 {
                return Err(format!("NetInfo::update: Failed to parse /proc/net/dev, \"{}\" device line is too short", self.dev_name).into());
            }

            self.down = entries[1].parse()?;
            self.up = entries[9].parse()?;
        } else {
            return Err(format!(
                "NetInfo::update: Failed to parse /proc/net/dev, can't find net device \"{}\"",
                self.dev_name
            )
            .into());
        }

        Ok(())
    }

    pub fn get_netstring(&mut self) -> Result<String, Error> {
        let down_diff: f64 = (self.down - self.prev_down) as f64;
        self.prev_down = self.down;
        let up_diff: f64 = (self.up - self.prev_up) as f64;
        self.prev_up = self.up;

        let mut output = String::new();
        if down_diff > 1024.0 * 1024.0 {
            write!(&mut output, "{:.2} MiB ", down_diff / (1024.0 * 1024.0))?;
        } else if down_diff > 1024.0 {
            write!(&mut output, "{:.2} KiB ", down_diff / 1024.0)?;
        } else {
            write!(&mut output, "{:.0} B ", down_diff)?;
        }

        if up_diff > 1024.0 * 1024.0 {
            write!(&mut output, "{:.2} MiB", up_diff / (1024.0 * 1024.0))?;
        } else if up_diff > 1024.0 {
            write!(&mut output, "{:.2} KiB", up_diff / 1024.0)?;
        } else {
            write!(&mut output, "{:.0} B", up_diff)?;
        }

        Ok(output)
    }
}

pub fn get_meminfo() -> Result<String, Error> {
    let mut meminfo_string = String::new();
    {
        let mut meminfo: File = File::open("/proc/meminfo")?;
        meminfo.read_to_string(&mut meminfo_string)?;
    }

    let mut is_total_mega = false;
    let mut total: u32 = 0;
    let mut available: u32 = 0;
    for line in meminfo_string.lines() {
        if line.starts_with("MemTotal:") {
            let line_parts = line
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            total = line_parts[1].parse()?;
        } else if line.starts_with("MemAvailable:") {
            let line_parts = line
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            available = line_parts[1].parse()?;
        }
    }

    let mut used: f64 = (total - available) as f64;
    let mut is_used_mega = false;

    if total == 0 {
        Ok("0".into())
    } else {
        let mut total = total as f64;

        if total > 1024.0 {
            total /= 1024.0;
            is_total_mega = true;
        }

        if used > 1024.0 {
            used /= 1024.0;
            is_used_mega = true;
        }

        let mut output: String;
        if is_used_mega {
            output = format!("{:.2} ", used);
            output.push_str("MiB / ");
        } else {
            output = format!("{:.0} ", used);
            output.push_str("KiB / ");
        }

        if is_total_mega {
            write!(&mut output, "{:.2} ", total)?;
            output.push_str("MiB");
        } else {
            write!(&mut output, "{:.0} ", total)?;
            output.push_str("KiB");
        }

        Ok(output)
    }
}

pub fn get_loadavg() -> Result<String, Error> {
    let mut loadavg_string = String::new();
    {
        let mut loadavg_file: File = File::open("/proc/loadavg")?;
        loadavg_file.read_to_string(&mut loadavg_string)?;
    }

    let loadavg_parts: Vec<&str> = loadavg_string.split_whitespace().collect();
    if loadavg_parts.len() < 3 {
        return Err("loadavg: failed to parse".to_owned().into());
    }

    Ok(format!(
        "{} {} {}",
        loadavg_parts[0], loadavg_parts[1], loadavg_parts[2]
    ))
}
