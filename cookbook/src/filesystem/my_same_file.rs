use same_file::Handle;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

pub fn same_file_rw() -> Result<(), Error> {
    let path_to_read = Path::new("new.txt");

    let stdout_handle = Handle::stdout()?;
    let handle = Handle::from_path(path_to_read)?;

    if handle == stdout_handle {
        return Err(Error::new(
            ErrorKind::Other,
            "You're reading and writing to the same file",
        ));
    } else {
        let file = File::open(path_to_read)?;
        let file = BufReader::new(file);
        for (num, line) in file.lines().enumerate() {
            println!("{}: {}", num, line?.to_uppercase());
        }
    }

    Ok(())
}
