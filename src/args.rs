use std::collections::HashMap;
use std::io;
use std::io::Write;

pub struct ArgsResult {
    pub map: HashMap<String, String>,
    pub regex_cmds: Vec<String>,
}

pub fn get_args() -> ArgsResult {
    let mut map = HashMap::new();
    let mut regex_cmds = Vec::new();

    let mut first = true;
    for arg in std::env::args() {
        if first {
            first = false;
            continue;
        } else if arg.starts_with("--netdev=") {
            let (_, back) = arg.split_at(9);
            map.insert("netdev".into(), back.into());
        } else if arg.starts_with("--netdev_width=") {
            let (_, back) = arg.split_at(15);
            map.insert("netdevwidth".into(), back.into());
        } else if arg.starts_with("--netgraph_max_bytes=") {
            let (_, back) = arg.split_at(21);
            map.insert("netgraph".into(), back.into());
        } else if arg.starts_with("--netgraph_dyn_display") {
            map.insert("netgraph-dyndisplay".into(), String::new());
        } else if arg.starts_with("--interval-sec=") {
            let (_, back) = arg.split_at(15);
            map.insert("interval-sec".into(), back.into());
        } else if arg == "--acpi-builtin" {
            map.insert("acpi-builtin".into(), String::new());
        } else if arg.starts_with("--regex-cmd=") {
            let (_, back) = arg.split_at(12);
            regex_cmds.push(back.to_owned());
        } else if arg.starts_with("--time-format=") {
            let (_, back) = arg.split_at(14);
            map.insert("time-format".into(), back.to_owned());
        } else if arg == "--help" || arg == "-h" {
            map.insert("help".into(), "".into());
        } else {
            let mut stderr_handle = io::stderr().lock();
            stderr_handle
                .write_all(format!("WARNING: Got invalid arg \"{}\"!\n", arg).as_bytes())
                .ok();
        }
    }

    ArgsResult { map, regex_cmds }
}

pub fn print_usage() {
    let mut stderr_handle = io::stderr().lock();
    stderr_handle.write_all(b"Usage:\n").ok();
    stderr_handle
        .write_all(b"  -h | --help\t\t\t\t\t\tPrints help\n")
        .ok();
    stderr_handle
        .write_all(b"  --netdev=<device_name>\t\t\t\tCheck network traffic on specified device\n")
        .ok();
    stderr_handle
        .write_all(b"  --netdev_width=<width>\t\t\t\tSets the min-width of the netdev output (default 11)\n")
        .ok();
    stderr_handle
        .write_all(b"  --netgraph_max_bytes=<bytes>\t\t\t\tEnable \"graph\" output when polling network traffic\n")
        .ok();
    stderr_handle
        .write_all(b"                              \t\t\t\t  (Set to \"dynamic\" instead of a byte count for dynamic sizing)\n")
        .ok();
    stderr_handle
        .write_all(b"  --netgraph_dyn_display\t\t\t\tEnable showing the current maximum value in the graph\n")
        .ok();
    stderr_handle
        .write_all(
            b"  --interval-sec=<seconds>\t\t\t\tOutput at intervals of <seconds> (default 5)\n",
        )
        .ok();
    stderr_handle
        .write_all(
            b"  --acpi-builtin\t\t\t\t\tUse \"acpi -b\" built-in fetching (battery info, with color)\n",
        )
        .ok();
    stderr_handle
        .write_all(
            b"  --regex-cmd=<cmd>[SPLIT]<args...>[SPLIT]<regex>\tUse an output of a command as a metric\n",
        )
        .ok();
    stderr_handle
        .write_all(
            b"  --time-format=<date format string>\t\t\tSet the format string for the date\n",
        )
        .ok();
}
