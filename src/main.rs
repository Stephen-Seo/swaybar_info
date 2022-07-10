mod args;
mod proc;
mod swaybar_object;

use std::io::{self, Write};
use std::time::Duration;
use swaybar_object::*;

fn main() {
    let args_map = args::get_args();
    if args_map.contains_key("help") {
        args::print_usage();
        return;
    }

    let mut net_obj: Option<proc::NetInfo> = None;
    let mut interval: Duration = Duration::from_secs(5);
    if args_map.contains_key("netdev") {
        net_obj = Some(proc::NetInfo::new(
            args_map.get("netdev").unwrap().to_owned(),
        ));
    }
    if args_map.contains_key("interval-sec") {
        let seconds: Result<i64, _> = args_map.get("interval-sec").unwrap().parse();
        if let Ok(seconds_value) = seconds {
            if seconds_value > 0 {
                interval = Duration::from_secs(seconds_value as u64);
            } else {
                let mut stderr_handle = io::stderr().lock();
                stderr_handle
                    .write_all(
                        format!(
                            "WARNING: Invalid --interval-sec=\"{}\", defaulting to 5!\n",
                            seconds_value
                        )
                        .as_bytes(),
                    )
                    .ok();
            }
        } else {
            let mut stderr_handle = io::stderr().lock();
            stderr_handle
                .write_all(b"WARNING: Failed to parse --interval-sec=?, defaulting to 5!\n")
                .ok();
        }
    }

    println!(
        "{}",
        serde_json::to_string(&swaybar_object::SwaybarHeader::new())
            .expect("Should be able to serialize SwaybarHeader")
    );
    println!("[");

    loop {
        let mut array = SwaybarArray::new();

        // network traffic
        if let Some(net) = &mut net_obj {
            if let Err(e) = net.update() {
                let mut stderr_handle = io::stderr().lock();
                stderr_handle.write_all(e.to_string().as_bytes()).ok();
                net_obj = None;
            } else {
                let netinfo_result = net.get_netstring();
                if let Err(e) = netinfo_result {
                    let mut stderr_handle = io::stderr().lock();
                    stderr_handle.write_all(e.to_string().as_bytes()).ok();
                } else {
                    let netinfo_string = netinfo_result.unwrap();
                    let netinfo_parts: Vec<&str> = netinfo_string.split_whitespace().collect();

                    {
                        let mut down_object = SwaybarObject::from_string(format!(
                            "{} {}",
                            netinfo_parts[0], netinfo_parts[1]
                        ));
                        down_object.color = Some("#ff8888ff".into());
                        array.push_object(down_object);
                    }

                    {
                        let mut up_object = SwaybarObject::from_string(format!(
                            "{} {}",
                            netinfo_parts[2], netinfo_parts[3]
                        ));
                        up_object.color = Some("#88ff88ff".into());
                        array.push_object(up_object);
                    }
                }
            }
        }

        // meminfo
        {
            let meminfo_result = proc::get_meminfo();
            let meminfo_string: String = if let Err(e) = meminfo_result {
                let mut stderr_handle = io::stderr().lock();
                stderr_handle.write_all(format!("{}\n", e).as_bytes()).ok();
                String::from("MEMINFO ERROR")
            } else {
                meminfo_result.unwrap()
            };
            let meminfo_obj = SwaybarObject::from_string(meminfo_string);
            array.push_object(meminfo_obj);
        }

        // loadavg
        {
            let loadavg_result = proc::get_loadavg();
            let loadavg_string: String = if let Err(e) = loadavg_result {
                let mut stderr_handle = io::stderr().lock();
                stderr_handle.write_all(format!("{}\n", e).as_bytes()).ok();
                String::from("LOADAVG ERROR")
            } else {
                loadavg_result.unwrap()
            };
            let loadavg_obj = SwaybarObject::from_string(loadavg_string);
            array.push_object(loadavg_obj);
        }

        // time
        {
            array.push_object(SwaybarObject::default());
        }

        println!("{}", array);
        std::thread::sleep(interval);
    }
}
