use super::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn read_lines() -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    //for line in buffered.lines().filter_map(|l| l.ok()) {
    //    println!("{:?}", line);
    //}

    for line in buffered.lines() {
        println!("{:?}", line?);
    }

    Ok(())
}
