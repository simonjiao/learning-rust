use sahw4::area::{area, Circle, Shape, Square, Triangle};
use std::env;

fn main() {
    let args = env::args().map(|s| s.to_lowercase()).collect::<Vec<_>>();
    if args.len() < 3 {
        println!("{} SHAPE [p0 p1 ...]", args[0]);
        return;
    }
    match args[1].as_str() {
        "circle" => {
            let radius = args[2].parse::<f64>().expect("invalid radius");
            let x = Circle::new(radius);
            if !x.valid() {
                println!("invalid shape {:?}", x);
                return;
            }
            println!("{:?}: {}", x, area(&x));
        }
        "triangle" => {
            if args.len() < 5 {
                println!("need for edges for a triangle");
                return;
            }
            let e0 = args[2].parse::<f64>().expect("invalid length of a edge");
            let e1 = args[3].parse::<f64>().expect("invalid length of a edge");
            let e2 = args[4].parse::<f64>().expect("invalid length of a edge");

            let x = Triangle::new(e0, e1, e2);
            if !x.valid() {
                println!("invalid shape {:?}", x);
                return;
            }
            println!("{:?}: {}", x, area(&x));
        }
        "square" => {
            let e = args[2].parse::<f64>().expect("invalid length of a edge");
            let x = Square::new(e);
            if !x.valid() {
                println!("invalid shape {:?}", x);
                return;
            }
            println!("{:?}: {}", x, area(&x));
        }
        _ => {
            println!("Invalid shape {}", args[1]);
            return;
        }
    };
}
