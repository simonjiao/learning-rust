use env_logger::WriteStyle;
use learning_rust::logging;
use log::LevelFilter;

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .write_style(WriteStyle::Always)
        .init();
    //env_logger::init();
    logging::it_works();
    log::info!("INFO");
    println!("call logging")
}
