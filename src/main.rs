mod proc;
mod swaybar_object;

fn main() {
    println!(
        "{}",
        serde_json::to_string(&swaybar_object::SwaybarHeader::new())
            .expect("Should be able to serialize SwaybarHeader")
    );
    println!("[");
    let mut array = swaybar_object::SwaybarArray::new();
    array.push_object(swaybar_object::SwaybarObject::default());
    array.push_object(swaybar_object::SwaybarObject::new());
    array.push_object(swaybar_object::SwaybarObject::default());
    let meminfo_string = proc::get_meminfo().expect("Should be able to get meminfo");
    let meminfo_object = swaybar_object::SwaybarObject::from_string(meminfo_string);
    array.push_object(meminfo_object);
    println!("{}", array);
}
