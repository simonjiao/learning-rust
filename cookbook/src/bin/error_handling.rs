use learning_rust::avoid_discarding;
use learning_rust::error_backtrace;
use learning_rust::read_uptime;

//pub mod memory_management;

fn main() {
    match read_uptime() {
        Ok(time) => println!("uptime {}", time),
        Err(e) => println!("Error: {}", e),
    }

    if let Err(error) = avoid_discarding::run() {
        use avoid_discarding::ErrorKind;
        match error.kind() {
            ErrorKind::Io(_) => println!("Standard IO error: {}", error),
            ErrorKind::Reqwest(_) => println!("Reqwest error: {}", error),
            ErrorKind::ParseIntError(_) => println!("Standard parse int error: {}", error),
            ErrorKind::RandomResponseError(_) => println!("User defined error: {:?}", error),
            _ => println!("Other error: {:?}", error),
        }
    }

    error_backtrace::backtrace();
}
