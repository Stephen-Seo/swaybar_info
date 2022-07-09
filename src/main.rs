mod swaybar_object;

fn main() {
    println!("Hello, world!");

    let mut array = swaybar_object::SwaybarArray::new();
    array.push_object(swaybar_object::SwaybarObject::default());
    array.push_object(swaybar_object::SwaybarObject::new());
    array.push_object(swaybar_object::SwaybarObject::default());
    println!("{}", array);
}
