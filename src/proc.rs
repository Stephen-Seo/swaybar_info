use std::fmt::Write as FMTWrite;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write as IOWrite;

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
    graph: String,
    graph_history: Vec<f64>,
    down: u64,
    prev_down: u64,
    up: u64,
    prev_up: u64,
    first_iteration: bool,
}

impl NetInfo {
    pub fn new(dev_name: String, graph_size_opt: Option<usize>) -> Self {
        let mut s = Self {
            dev_name,
            graph: String::from(" "),
            graph_history: Vec::new(),
            down: 0,
            prev_down: 0,
            up: 0,
            prev_up: 0,
            first_iteration: true,
        };

        if let Some(graph_size) = graph_size_opt {
            if graph_size > 0 {
                s.graph_history.resize(graph_size, 0.0);
                s.graph = s.graph.repeat(graph_size);
            } else {
                let mut stderr_handle = std::io::stderr().lock();
                stderr_handle
                    .write_all(
                        "WARNING: Invalid graph_size value passed to NetInfo, ignoring...\n"
                            .as_bytes(),
                    )
                    .ok();
                s.graph_history.resize(10, 0.0);
                s.graph = s.graph.repeat(10);
            }
        } else {
            s.graph_history.resize(10, 0.0);
            s.graph = s.graph.repeat(10);
        }

        s
    }

    pub fn update(&mut self) -> Result<(), Error> {
        let mut netdev_string = String::new();
        {
            let mut netdev_file: File = File::open("/proc/net/dev")?;
            netdev_file.read_to_string(&mut netdev_string)?;
        }

        let mut dev_line: Option<&str> = None;
        for line in netdev_string.lines().map(|line| line.trim()) {
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

            if !self.first_iteration {
                self.down = entries[1].parse()?;
                self.up = entries[9].parse()?;
            } else {
                self.prev_down = entries[1].parse()?;
                self.prev_up = entries[9].parse()?;
            }
        } else {
            return Err(format!(
                "NetInfo::update: Failed to parse /proc/net/dev, can't find net device \"{}\"",
                self.dev_name
            )
            .into());
        }

        self.first_iteration = false;

        Ok(())
    }

    // Returns netinfo down/up, graph, and history_max (if dynamic is enabled)
    pub fn get_netstring(
        &mut self,
        graph_max_opt: Option<f64>,
    ) -> Result<(String, String, String), Error> {
        let down_diff: f64 = if self.down > self.prev_down {
            let value = (self.down - self.prev_down) as f64;
            self.prev_down = self.down;
            value
        } else {
            0.0
        };
        let up_diff: f64 = if self.up > self.prev_up {
            let value = (self.up - self.prev_up) as f64;
            self.prev_up = self.up;
            value
        } else {
            0.0
        };

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

        let diff_max = if down_diff > up_diff {
            down_diff
        } else {
            up_diff
        };

        let mut diff_max_string = String::new();

        if let Some(graph_max) = graph_max_opt {
            let graph_value: u8 = if diff_max > graph_max {
                8
            } else {
                (diff_max / graph_max * 8.0f64).round() as u8
            };

            self.graph.remove(0);
            match graph_value {
                0 => self.graph.push(' '),
                1 => self.graph.push('▁'),
                2 => self.graph.push('▂'),
                3 => self.graph.push('▃'),
                4 => self.graph.push('▄'),
                5 => self.graph.push('▅'),
                6 => self.graph.push('▆'),
                7 => self.graph.push('▇'),
                _ => self.graph.push('█'),
            }
        } else {
            self.graph_history.rotate_left(1);
            {
                let end_idx = self.graph_history.len() - 1;
                self.graph_history[end_idx] = diff_max;
            }

            let mut history_max: f64 = 0.0;
            for value in &self.graph_history {
                if history_max < *value {
                    history_max = *value;
                }
            }

            if history_max > 1024.0 * 1024.0 {
                write!(
                    &mut diff_max_string,
                    "{:.2} MiB",
                    history_max / (1024.0 * 1024.0)
                )?;
            } else if history_max > 1024.0 {
                write!(&mut diff_max_string, "{:.2} KiB", history_max / 1024.0)?;
            } else {
                write!(&mut diff_max_string, "{:.0} B", history_max)?;
            }

            self.graph.clear();
            if history_max == 0.0 {
                self.graph = String::from(" ").repeat(self.graph_history.len());
            } else {
                for value in &self.graph_history {
                    match (8.0 * value / history_max).round() as u8 {
                        0 => self.graph.push(' '),
                        1 => self.graph.push('▁'),
                        2 => self.graph.push('▂'),
                        3 => self.graph.push('▃'),
                        4 => self.graph.push('▄'),
                        5 => self.graph.push('▅'),
                        6 => self.graph.push('▆'),
                        7 => self.graph.push('▇'),
                        _ => self.graph.push('█'),
                    }
                }
            }
        }

        Ok((output, self.graph.clone(), diff_max_string))
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
