use std::fs::File;
use std::io;
use std::io::prelude::*;

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

    pub fn update(&mut self) -> io::Result<()> {
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
                return Err(io::Error::new(io::ErrorKind::Other, format!("NetInfo::update: Failed to parse /proc/net/dev, \"{}\" device line is too short", self.dev_name)));
            }

            self.down = entries[1].parse().map_err(|_| io::Error::new(io::ErrorKind::Other, format!("NetInfo::update: Failed to parse recv bytes in /proc/net/dev for device \"{}\"", self.dev_name)))?;
            self.up = entries[9].parse().map_err(|_| io::Error::new(io::ErrorKind::Other, format!("NetInfo::update: Failed to parse recv bytes in /proc/net/dev for device \"{}\"", self.dev_name)))?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "NetInfo::update: Failed to parse /proc/net/dev, can't find net device \"{}\"",
                    self.dev_name
                ),
            ));
        }

        Ok(())
    }

    pub fn get_netstring(&mut self) -> String {
        let down_diff = self.down - self.prev_down;
        self.prev_down = self.down;
        let up_diff = self.up - self.prev_up;
        self.prev_up = self.up;

        let mut output = String::new();
        if down_diff > 1024 * 1024 {
            output.push_str(&format!("{} MiB ", down_diff / 1024 / 1024));
        } else if down_diff > 1024 {
            output.push_str(&format!("{} KiB ", down_diff / 1024));
        } else {
            output.push_str(&format!("{} B ", down_diff));
        }

        if up_diff > 1024 * 1024 {
            output.push_str(&format!("{} MiB", up_diff / 1024 / 1024));
        } else if up_diff > 1024 {
            output.push_str(&format!("{} KiB", up_diff / 1024));
        } else {
            output.push_str(&format!("{} B", up_diff));
        }

        output
    }
}

pub fn get_meminfo() -> io::Result<String> {
    let mut meminfo_string = String::new();
    {
        let mut meminfo: File = File::open("/proc/meminfo")?;
        meminfo.read_to_string(&mut meminfo_string)?;
    }

    let mut is_total_giga = false;
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
    let mut is_used_giga = false;

    if total == 0 {
        Ok("0".into())
    } else {
        if total > 1024 {
            total /= 1024;
            is_total_giga = true;
        }

        if used > 1024 {
            used /= 1024;
            is_used_giga = true;
        }

        let mut output = format!("{} ", used);
        if is_used_giga {
            output.push_str("GiB / ");
        } else {
            output.push_str("KiB / ");
        }

        output.push_str(&format!("{} ", total));
        if is_total_giga {
            output.push_str("GiB");
        } else {
            output.push_str("KiB");
        }

        Ok(output)
    }
}

pub fn get_loadavg() -> io::Result<String> {
    let mut loadavg_string = String::new();
    {
        let mut loadavg_file: File = File::open("/proc/loadavg")?;
        loadavg_file.read_to_string(&mut loadavg_string)?;
    }

    let loadavg_parts: Vec<&str> = loadavg_string.split_whitespace().collect();
    if loadavg_parts.len() < 3 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "loadavg: failed to parse",
        ));
    }

    Ok(format!(
        "{} {} {}",
        loadavg_parts[0], loadavg_parts[1], loadavg_parts[2]
    ))
}
