use std::collections::HashMap;

pub fn get_args() -> HashMap<String, String> {
    let mut map = HashMap::new();

    for arg in std::env::args() {
        if arg.starts_with("--netdev=") {
            let (_, back) = arg.split_at(9);
            map.insert("netdev".into(), back.into());
        } else if arg.starts_with("--interval-sec=") {
            let (_, back) = arg.split_at(15);
            map.insert("interval-sec".into(), back.into());
        } else if arg.starts_with("--help") || arg.starts_with("-h") {
            map.insert("help".into(), "".into());
        }
    }

    map
}

pub fn print_usage() {
    println!("Usage:");
    println!("  --help\t\t\tPrints help");
    println!("  --netdev=<device_name>\tCheck network traffic on specified device");
    println!("  --interval-sec=<seconds>\tOutput at intervals of <seconds> (default 5)");
}
