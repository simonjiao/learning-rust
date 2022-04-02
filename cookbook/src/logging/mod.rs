use log::{debug, info, warn};

pub fn it_works() {
    debug!("debug");
    info!("info");
    warn!("warn")
}

#[cfg(test)]
mod tests {
    use log::{debug, info, warn};

    #[test]
    fn it_works() {
        env_logger::init();

        debug!("debug");
        info!("info");
        warn!("warn")
    }
}
