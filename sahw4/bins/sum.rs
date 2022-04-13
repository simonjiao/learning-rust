use sahw4::sum::sum;

fn main() {
    let numbers = std::env::args()
        .skip(1)
        .filter_map(|n| match n.parse::<u32>() {
            Ok(num) => Some(num),
            Err(_) => {
                println!("skip {}", n);
                None
            }
        })
        .collect::<Vec<_>>();

    println!("{:?}", sum(numbers.as_slice()));
}
