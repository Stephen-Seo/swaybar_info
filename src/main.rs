mod args;
mod proc;
mod swaybar_object;

fn main() {
    let args_map = args::get_args();

    let mut net_obj = None;
    if args_map.contains_key("netdev") {
        net_obj = Some(proc::NetInfo::new(
            args_map.get("netdev").unwrap().to_owned(),
        ));
    }

    println!(
        "{}",
        serde_json::to_string(&swaybar_object::SwaybarHeader::new())
            .expect("Should be able to serialize SwaybarHeader")
    );
    println!("[");
    let mut array = swaybar_object::SwaybarArray::new();
    array.push_object(swaybar_object::SwaybarObject::default());
    {
        let meminfo_string = proc::get_meminfo().expect("Should be able to get meminfo");
        let meminfo_object = swaybar_object::SwaybarObject::from_string(meminfo_string);
        array.push_object(meminfo_object);
    }
    {
        let loadavg_string = proc::get_loadavg().expect("Should be able to get loadavg");
        let loadavg_object = swaybar_object::SwaybarObject::from_string(loadavg_string);
        array.push_object(loadavg_object);
    }

    if let Some(mut netinfo) = net_obj {
        for _i in 0..10 {
            netinfo.update().expect("netinfo.update() shouldn't fail");
            let netinfo_string = netinfo.get_netstring();
            array.push_object(swaybar_object::SwaybarObject::from_string(netinfo_string));
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    println!("{}", array);
}
