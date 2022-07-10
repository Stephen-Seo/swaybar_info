use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn get_args() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut first = true;
    for arg in std::env::args() {
        if first {
            first = false;
            continue;
        } else if arg.starts_with("--netdev=") {
            let (_, back) = arg.split_at(9);
            map.insert("netdev".into(), back.into());
        } else if arg.starts_with("--interval-sec=") {
            let (_, back) = arg.split_at(15);
            map.insert("interval-sec".into(), back.into());
        } else if arg.starts_with("--help") || arg.starts_with("-h") {
            map.insert("help".into(), "".into());
        } else {
            let mut stderr_handle = io::stderr().lock();
            stderr_handle
                .write_all(format!("WARNING: Got invalid arg \"{}\"!\n", arg).as_bytes())
                .ok();
        }
    }

    map
}

pub fn print_usage() {
    let mut stderr_handle = io::stderr().lock();
    stderr_handle.write_all(b"Usage:\n").ok();
    stderr_handle.write_all(b"  --help\t\t\tPrints help\n").ok();
    stderr_handle
        .write_all(b"  --netdev=<device_name>\tCheck network traffic on specified device\n")
        .ok();
    stderr_handle
        .write_all(b"  --interval-sec=<seconds>\tOutput at intervals of <seconds> (default 5)\n")
        .ok();
}
