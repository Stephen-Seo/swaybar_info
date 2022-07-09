use std::fs::File;
use std::io;
use std::io::prelude::*;

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
