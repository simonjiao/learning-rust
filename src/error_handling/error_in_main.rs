use error_chain::error_chain;
use std::fs::File;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ParseInt(std::num::ParseIntError);
    }
}
// type Result<T> =  std::result::Result<T, Error>

pub fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    //44361.46 344555.14

    Ok(uptime
        .split('.')
        .next()
        .ok_or("Cannot parse uptime data")?
        .parse()?)
}
