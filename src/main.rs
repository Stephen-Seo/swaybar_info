mod args;
mod builtin;
mod external;
mod proc;
mod swaybar_object;

use std::fmt::Write as FMTWrite;
use std::io::{self, Write};
use std::time::Duration;
use swaybar_object::*;

const DEFAULT_FMT_STRING: &str = "%F %r";

fn main() {
    let args_result = args::get_args();
    if args_result.map.contains_key("help") {
        args::print_usage();
        return;
    }

    let mut cmds: Vec<(&str, Vec<&str>, regex::Regex)> = Vec::new();
    for regex_cmd in &args_result.regex_cmds {
        let mut split_strs = regex_cmd.split_terminator("[SPLIT]");
        let cmd: &str = split_strs.next().expect("Should have cmd in option");
        let mut args: Vec<&str> = Vec::new();
        let mut next: Option<&str>;
        loop {
            next = split_strs.next();
            if let Some(str) = next {
                args.push(str);
            } else {
                break;
            }
        }
        if args.is_empty() {
            panic!("Missing regex for --regex-cmd=<cmd>,<args...>,<regex>");
        }

        let regex_str: &str = args[args.len() - 1];
        args.pop();

        let regex = regex::Regex::new(regex_str).expect("Should be able to compile regex");

        cmds.push((cmd, args, regex));
    }

    let mut net_obj: Option<proc::NetInfo> = None;
    let mut net_width: Option<u16> = Some(11);
    let mut net_graph_max: Option<f64> = None;
    let mut net_graph_is_dynamic: bool = false;
    let mut net_graph_show_dynamic_max: bool = false;
    let mut interval: Duration = Duration::from_secs(5);
    let mut net_graph_size: Option<usize> = None;
    if args_result.map.contains_key("netdev") {
        if let Some(size_str) = args_result.map.get("netgraph-size") {
            if let Ok(size) = size_str.parse::<usize>() {
                if size > 0 {
                    net_graph_size = Some(size);
                } else {
                    let mut stderr_handle = io::stderr().lock();
                    stderr_handle
                        .write_all(
                            "WARNING: Invalid value passed to --netgraph_size=..., ignoring...\n"
                                .as_bytes(),
                        )
                        .ok();
                }
            } else {
                let mut stderr_handle = io::stderr().lock();
                stderr_handle
                    .write_all(
                        "WARNING: Invalid value passed to --netgraph_size=..., ignoring...\n"
                            .as_bytes(),
                    )
                    .ok();
            }
        }
        net_obj = Some(proc::NetInfo::new(
            args_result.map.get("netdev").unwrap().to_owned(),
            net_graph_size,
        ));
    }
    if net_graph_size.is_none() {
        net_graph_size = Some(10);
    }

    if args_result.map.contains_key("netdevwidth") {
        let width_result: Result<u16, _> = args_result.map.get("netdevwidth").unwrap().parse();
        if let Ok(width) = width_result {
            net_width = Some(width);
        } else {
            let mut stderr_handle = io::stderr().lock();
            stderr_handle
                .write_all(
                    "WARNING: Invalid value passed to --netdev_width=..., ignoring...\n".as_bytes(),
                )
                .ok();
        }
    }
    if args_result.map.contains_key("netgraph") {
        if args_result.map.get("netgraph").as_ref().unwrap() == &"dynamic" {
            net_graph_is_dynamic = true;
        } else {
            let graph_max_result: Result<f64, _> = args_result.map.get("netgraph").unwrap().parse();
            if let Ok(graph_max) = graph_max_result {
                net_graph_max = Some(graph_max);
            } else {
                let mut stderr_handle = io::stderr().lock();
                stderr_handle
                    .write_all(
                        "WARNING: Invalid value passed to --netgraph_max_bytes=..., ignoring...\n"
                            .as_bytes(),
                    )
                    .ok();
            }
        }
    }
    if args_result.map.contains_key("netgraph-dyndisplay") {
        net_graph_show_dynamic_max = true;
    }
    if args_result.map.contains_key("interval-sec") {
        let seconds: Result<i64, _> = args_result.map.get("interval-sec").unwrap().parse();
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

    let mut batt_info: builtin::BattInfo = Default::default();
    let batt_info_enabled: bool = args_result.map.contains_key("acpi-builtin");
    let mut batt_info_error: bool = false;

    let mut time_fmt_str = DEFAULT_FMT_STRING;
    if let Some(s) = args_result.map.get("time-format") {
        time_fmt_str = s;
    }

    println!(
        "{}",
        serde_json::to_string(&swaybar_object::SwaybarHeader::new())
            .expect("Should be able to serialize SwaybarHeader")
    );
    println!("[");

    let mut array = SwaybarArray::new();
    let set_net_error = |is_empty: bool, array: &mut SwaybarArray, graph_max_opt: &Option<f64>| {
        if is_empty {
            if net_graph_is_dynamic && net_graph_show_dynamic_max {
                array.push_object(SwaybarObject::from_error_string(
                    "net_graph_dyn_max".to_owned(),
                    "net ERROR".into(),
                ));
            }

            if graph_max_opt.is_some() || net_graph_is_dynamic {
                array.push_object(SwaybarObject::from_error_string(
                    "net_graph".to_owned(),
                    "net ERROR".into(),
                ));
            }

            let down_obj =
                SwaybarObject::from_error_string("net_down".to_owned(), "Net ERROR".into());
            array.push_object(down_obj);

            let up_obj = SwaybarObject::from_error_string("net_up".to_owned(), "Net ERROR".into());
            array.push_object(up_obj);
        } else {
            if net_graph_is_dynamic && net_graph_show_dynamic_max {
                if let Some(dyn_max) = array.get_by_name_mut("net_graph_dyn_max") {
                    dyn_max.update_as_error("Net ERROR".to_owned());
                }
            }

            if graph_max_opt.is_some() || net_graph_is_dynamic {
                if let Some(graph_ref) = array.get_by_name_mut("net_graph") {
                    graph_ref.update_as_error("Net ERROR".to_owned());
                }
            }

            let down_ref_opt = array.get_by_name_mut("net_down");
            if let Some(down_ref) = down_ref_opt {
                down_ref.update_as_error("Net ERROR".to_owned());
            }

            let up_ref_opt = array.get_by_name_mut("net_up");
            if let Some(up_ref) = up_ref_opt {
                up_ref.update_as_error("Net ERROR".to_owned());
            }
        }
    };

    let handle_net = |is_empty: bool,
                      net: &mut proc::NetInfo,
                      array: &mut SwaybarArray|
     -> Result<(), proc::Error> {
        net.update()?;
        let (netinfo_string, graph_items, max_idx, history_max) =
            net.get_netstring(net_graph_max)?;
        let netinfo_parts: Vec<&str> = netinfo_string.split_whitespace().collect();

        if is_empty {
            if net_graph_is_dynamic && net_graph_show_dynamic_max {
                let mut graph_obj =
                    SwaybarObject::from_string("net_graph_dyn_max".to_owned(), history_max);
                graph_obj.color = Some("#ffff88".into());
                array.push_object(graph_obj);
            }

            if net_graph_max.is_some() || net_graph_is_dynamic {
                let mut graph_obj = SwaybarObject::from_string(
                    "net_graph".to_owned(),
                    " ".to_owned().repeat(net_graph_size.unwrap()),
                );
                graph_obj.markup = Some("pango".to_owned());
                array.push_object(graph_obj);
            }

            let mut width_string: Option<String> = None;
            if let Some(width) = net_width {
                let mut string = String::with_capacity(width.into());
                for _ in 0..width {
                    string.push('0');
                }
                width_string = Some(string);
            }

            {
                let mut down_object = SwaybarObject::from_string(
                    "net_down".to_owned(),
                    format!("{} {}", netinfo_parts[0], netinfo_parts[1]),
                );
                down_object.color = Some("#ff8888ff".into());
                down_object.min_width = width_string.clone();
                down_object.align = Some(String::from("right"));
                array.push_object(down_object);
            }

            {
                let mut up_object = SwaybarObject::from_string(
                    "net_up".to_owned(),
                    format!("{} {}", netinfo_parts[2], netinfo_parts[3]),
                );
                up_object.color = Some("#88ff88ff".into());
                up_object.min_width = width_string;
                up_object.align = Some(String::from("right"));
                array.push_object(up_object);
            }
        } else {
            if net_graph_is_dynamic && net_graph_show_dynamic_max {
                if let Some(graph_obj) = array.get_by_name_mut("net_graph_dyn_max") {
                    graph_obj.full_text = history_max;
                    if (net_graph_max.is_some() || net_graph_is_dynamic) && !graph_items.is_empty()
                    {
                        match graph_items[max_idx].get_value_type() {
                            proc::GraphItemType::Download => {
                                graph_obj.color = Some("#ff8888ff".into())
                            }
                            proc::GraphItemType::Upload => {
                                graph_obj.color = Some("#88ff88ff".into())
                            }
                            proc::GraphItemType::Both => graph_obj.color = Some("#ffff88ff".into()),
                        }
                    }
                }
            }

            if net_graph_max.is_some() || net_graph_is_dynamic {
                if let Some(graph_obj) = array.get_by_name_mut("net_graph") {
                    let mut text = String::new();
                    for item in graph_items.iter() {
                        match item.get_value_type() {
                            proc::GraphItemType::Download => {
                                write!(
                                    &mut text,
                                    "<span color=\"#ff8888ff\">{}</span>",
                                    item.get_value()
                                )?;
                            }
                            proc::GraphItemType::Upload => {
                                write!(
                                    &mut text,
                                    "<span color=\"#88ff88ff\">{}</span>",
                                    item.get_value()
                                )?;
                            }
                            proc::GraphItemType::Both => {
                                write!(
                                    &mut text,
                                    "<span color=\"#ffff88ff\">{}</span>",
                                    item.get_value()
                                )?;
                            }
                        }
                    }
                    graph_obj.full_text = text;
                }
            }

            if let Some(down_object) = array.get_by_name_mut("net_down") {
                down_object
                    .update_as_net_down(format!("{} {}", netinfo_parts[0], netinfo_parts[1]));
            }

            if let Some(up_object) = array.get_by_name_mut("net_up") {
                up_object.update_as_net_up(format!("{} {}", netinfo_parts[2], netinfo_parts[3]));
            }
        }

        Ok(())
    };

    loop {
        let is_empty = array.is_empty();

        // network traffic
        if let Some(net) = net_obj.as_mut() {
            if let Err(e) = handle_net(is_empty, net, &mut array) {
                let mut stderr_handle = io::stderr().lock();
                stderr_handle.write_all(format!("{}\n", e).as_bytes()).ok();
                net_obj = None;
                set_net_error(is_empty, &mut array, &net_graph_max);
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
            if is_empty {
                let meminfo_obj = SwaybarObject::from_string("meminfo".to_owned(), meminfo_string);
                array.push_object(meminfo_obj);
            } else if let Some(meminfo_obj) = array.get_by_name_mut("meminfo") {
                meminfo_obj.update_as_generic(meminfo_string, None);
            }
        }

        // regex_cmds
        {
            for (idx, (cmd, args, regex)) in cmds.iter().enumerate() {
                let cmd_result = external::get_cmd_output(cmd, args, regex);
                if let Ok(cmd_struct) = cmd_result {
                    if is_empty {
                        let mut cmd_obj = SwaybarObject::from_string(
                            format!("regex_cmd_{}", idx),
                            cmd_struct.matched,
                        );
                        cmd_obj.color = cmd_struct.color;
                        array.push_object(cmd_obj);
                    } else if let Some(cmd_obj) =
                        array.get_by_name_mut(&format!("regex_cmd_{}", idx))
                    {
                        cmd_obj.update_as_generic(cmd_struct.matched, cmd_struct.color);
                    }
                } else if let Err(e) = cmd_result {
                    let mut stderr_handle = io::stderr().lock();
                    stderr_handle.write_all(format!("{}\n", e).as_bytes()).ok();
                    if is_empty {
                        let cmd_obj = SwaybarObject::from_error_string(
                            format!("regex_cmd_{}", idx),
                            "REGEX_CMD ERROR".into(),
                        );
                        array.push_object(cmd_obj);
                    } else if let Some(cmd_obj) =
                        array.get_by_name_mut(&format!("regex_cmd_{}", idx))
                    {
                        cmd_obj.update_as_error("REGEX_CMD ERROR".into());
                    }
                }
            }
        }

        // batt_info
        if batt_info_enabled {
            if is_empty {
                let mut new_object = SwaybarObject::new("battinfo".to_owned());
                let result = batt_info.update(&mut new_object);
                if result.is_ok() {
                    array.push_object(new_object);
                } else {
                    new_object.update_as_error("BATTINFO ERROR".to_owned());
                    array.push_object(new_object);
                    batt_info_error = true;
                    let mut stderr_handle = io::stderr().lock();
                    stderr_handle
                        .write_all(format!("{}\n", result.unwrap_err()).as_bytes())
                        .ok();
                }
            } else if let Some(obj) = array.get_by_name_mut("battinfo") {
                if !batt_info_error {
                    let result = batt_info.update(obj);
                    if let Err(e) = result {
                        obj.update_as_error("BATTINFO ERROR".to_owned());
                        batt_info_error = true;
                        let mut stderr_handle = io::stderr().lock();
                        stderr_handle.write_all(format!("{}\n", e).as_bytes()).ok();
                    }
                }
            }
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
            if is_empty {
                let loadavg_obj = SwaybarObject::from_string("loadavg".to_owned(), loadavg_string);
                array.push_object(loadavg_obj);
            } else if let Some(loadavg_obj) = array.get_by_name_mut("loadavg") {
                loadavg_obj.update_as_generic(loadavg_string, None);
            }
        }

        // time
        if is_empty {
            let mut time_obj = SwaybarObject::new("current_time".to_owned());
            time_obj.update_as_date(time_fmt_str);
            array.push_object(time_obj);
        } else if let Some(time_obj) = array.get_by_name_mut("current_time") {
            time_obj.update_as_date(time_fmt_str);
        }

        println!("{}", array);
        std::thread::sleep(interval);
    }
}
