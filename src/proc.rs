use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    GenericError(String),
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self::IOError(io_error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(parse_error: std::num::ParseIntError) -> Self {
        Self::ParseIntError(parse_error)
    }
}

impl From<String> for Error {
    fn from(string: String) -> Self {
        Self::GenericError(string)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => e.fmt(f),
            Error::ParseIntError(e) => e.fmt(f),
            Error::GenericError(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => e.source(),
            Error::ParseIntError(e) => e.source(),
            _ => None,
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

        let mut dev_line: Option<String> = None;
        for line in netdev_string.lines() {
            if line.starts_with(&self.dev_name) {
                dev_line = Some(line.to_owned());
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

    pub fn get_netstring(&mut self) -> String {
        let down_diff: f64 = (self.down - self.prev_down) as f64;
        self.prev_down = self.down;
        let up_diff: f64 = (self.up - self.prev_up) as f64;
        self.prev_up = self.up;

        let mut output = String::new();
        if down_diff > 1024.0 * 1024.0 {
            output.push_str(&format!("{:.2} MiB ", down_diff / (1024.0 * 1024.0)));
        } else if down_diff > 1024.0 {
            output.push_str(&format!("{:.2} KiB ", down_diff / 1024.0));
        } else {
            output.push_str(&format!("{:.0} B ", down_diff));
        }

        if up_diff > 1024.0 * 1024.0 {
            output.push_str(&format!("{:.2} MiB", up_diff / (1024.0 * 1024.0)));
        } else if up_diff > 1024.0 {
            output.push_str(&format!("{:.2} KiB", up_diff / 1024.0));
        } else {
            output.push_str(&format!("{:.0} B", up_diff));
        }

        output
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
            total = line_parts[1]
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "MemTotal: parse error"))?;
        } else if line.starts_with("MemAvailable:") {
            let line_parts = line
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            available = line_parts[1]
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "MemAvailable: parse error"))?;
        }
    }

    let mut used = total - available;
    let mut is_used_mega = false;

    if total == 0 {
        Ok("0".into())
    } else {
        if total > 1024 {
            total /= 1024;
            is_total_mega = true;
        }

        if used > 1024 {
            used /= 1024;
            is_used_mega = true;
        }

        let mut output = format!("{} ", used);
        if is_used_mega {
            output.push_str("MiB / ");
        } else {
            output.push_str("KiB / ");
        }

        output.push_str(&format!("{} ", total));
        if is_total_mega {
            output.push_str("MiB");
        } else {
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
