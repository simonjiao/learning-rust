use sahw4::stoplights::StopLight;
use std::{env, str::FromStr};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if let Some(light) = args.get(1) {
        match StopLight::from_str(light.trim()) {
            Ok(color) => println!("{}: {}", color, color.duration()),
            Err(e) => println!("{}", e),
        };
    } else {
        println!("{} {}", StopLight::Red, StopLight::Red.duration());
        println!("{} {}", StopLight::Green, StopLight::Green.duration());
        println!("{} {}", StopLight::Yellow, StopLight::Yellow.duration());
    }
}
