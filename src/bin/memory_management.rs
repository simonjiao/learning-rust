use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PRIVILIGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["admin", "user"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

fn show_access(name: &str) {
    let access = PRIVILIGES.get(name);
    println!("{}: {:?}", name, access);
}

fn main() {
    let access = PRIVILIGES.get("James");
    println!("James: {:?}", access);

    show_access("Jim");
}
