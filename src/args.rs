use std::collections::HashMap;

pub fn get_args() -> HashMap<String, String> {
    let mut map = HashMap::new();

    for arg in std::env::args() {
        if arg.starts_with("--netdev=") {
            let (_, back) = arg.split_at(9);
            map.insert("netdev".into(), back.into());
        }
    }

    map
}
