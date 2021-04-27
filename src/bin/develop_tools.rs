fn main() {
    //use env_logger::{Builder, Target};
    //use learning_rust::execute_query;
    //use learning_rust::execute_query_err;
    //env_logger::init();
    //Builder::new().target(Target::Stdout).init();

    //log::error!("This error has been sent to stdout");

    //execute_query("DROP TABLE students");

    //if let Err(e) = execute_query_err("Drop") {
    //    log::error!("Failed to execute query: {}", e);
    //}

    //use learning_rust::ConsoleLogger;
    //use log::LevelFilter;

    //static CONSOLE_LOGGER:ConsoleLogger = ConsoleLogger;

    //if let Err(e) = log::set_logger(&CONSOLE_LOGGER) {
    //    println!("Failed to set logger {}", e)
    //}
    //log::set_max_level(LevelFilter::Info);

    //log::debug!("debug");
    //log::info!("info");
    //log::warn!("warn");
    //log::error!("error");

    use learning_rust::log_to_syslog;
    if let Err(e) = log_to_syslog() {
        println!("Error: {}", e);
    }
}
