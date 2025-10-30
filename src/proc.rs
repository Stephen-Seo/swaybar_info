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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GraphItemType {
    Download,
    Upload,
    Both,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GraphItem {
    value: char,
    num_value: f64,
    value_type: GraphItemType,
}

impl GraphItem {
    pub fn get_value(&self) -> char {
        self.value
    }

    pub fn set_value(&mut self, c: char) {
        self.value = c;
    }

    pub fn get_num_value(&self) -> f64 {
        self.num_value
    }

    pub fn get_value_type(&self) -> GraphItemType {
        self.value_type
    }
}

pub struct NetInfo {
    dev_name: String,
    graph: Vec<GraphItem>,
    down: u64,
    prev_down: u64,
    up: u64,
    prev_up: u64,
    first_iteration: bool,
    pub errored: bool,
}

impl NetInfo {
    pub fn new(dev_name: String, graph_size_opt: Option<usize>) -> Self {
        let mut s = Self {
            dev_name,
            graph: vec![GraphItem {
                value: ' ',
                num_value: 0.0,
                value_type: GraphItemType::Both,
            }],
            down: 0,
            prev_down: 0,
            up: 0,
            prev_up: 0,
            first_iteration: true,
            errored: false,
        };

        if let Some(graph_size) = graph_size_opt {
            if graph_size > 0 {
                s.graph = s.graph.repeat(graph_size);
            } else {
                let mut stderr_handle = std::io::stderr().lock();
                stderr_handle
                    .write_all(
                        "WARNING: Invalid graph_size value passed to NetInfo, ignoring...\n"
                            .as_bytes(),
                    )
                    .ok();
                s.graph = s.graph.repeat(10);
            }
        } else {
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

    // Returns netinfo down/up, graph, max idx, and history_max (if dynamic is enabled)
    pub fn get_netstring(
        &mut self,
        graph_max_opt: Option<f64>,
    ) -> Result<(String, &Vec<GraphItem>, usize, String), Error> {
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

        let mut graph_type = GraphItemType::Both;
        let diff_max = if down_diff > up_diff {
            graph_type = GraphItemType::Download;
            down_diff
        } else {
            if down_diff < up_diff {
                graph_type = GraphItemType::Upload;
            }
            up_diff
        };

        let mut diff_max_string = String::new();
        let mut history_max_idx = 0;

        if let Some(graph_max) = graph_max_opt {
            let graph_value: u8 = if diff_max > graph_max {
                8
            } else {
                (diff_max / graph_max * 8.0f64).round() as u8
            };

            self.graph.remove(0);
            match graph_value {
                0 => self.graph.push(GraphItem {
                    value: ' ',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
                1 => self.graph.push(GraphItem {
                    value: '▁',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
                2 => self.graph.push(GraphItem {
                    value: '▂',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
                3 => self.graph.push(GraphItem {
                    value: '▃',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
                4 => self.graph.push(GraphItem {
                    value: '▄',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
                5 => self.graph.push(GraphItem {
                    value: '▅',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
                6 => self.graph.push(GraphItem {
                    value: '▆',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
                7 => self.graph.push(GraphItem {
                    value: '▇',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
                _ => self.graph.push(GraphItem {
                    value: '█',
                    num_value: 0.0,
                    value_type: graph_type,
                }),
            }
        } else {
            self.graph.rotate_left(1);
            {
                let end_idx = self.graph.len() - 1;
                self.graph[end_idx] = GraphItem {
                    value: ' ',
                    num_value: diff_max,
                    value_type: graph_type,
                };
            }

            let mut history_max: f64 = 0.0;
            for (idx, value) in self
                .graph
                .iter()
                .map(|item| item.get_num_value())
                .enumerate()
            {
                if history_max < value {
                    history_max = value;
                    history_max_idx = idx;
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

            for item in self.graph.iter_mut() {
                match (8.0 * item.get_num_value() / history_max).round() as u8 {
                    0 => item.set_value(' '),
                    1 => item.set_value('▁'),
                    2 => item.set_value('▂'),
                    3 => item.set_value('▃'),
                    4 => item.set_value('▄'),
                    5 => item.set_value('▅'),
                    6 => item.set_value('▆'),
                    7 => item.set_value('▇'),
                    _ => item.set_value('█'),
                }
            }
        }

        Ok((output, &self.graph, history_max_idx, diff_max_string))
    }

    pub fn set_dev_name(&mut self, dev_name: &str) {
        self.dev_name = dev_name.to_owned();
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
