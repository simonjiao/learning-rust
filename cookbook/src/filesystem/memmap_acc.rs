use memmap::Mmap;
use std::fs::File;
use std::io::{Error, Write};

pub fn memmap_acc() -> Result<(), Error> {
    write!(
        File::create("content.txt")?,
        "My hovercraft is full of eels!"
    )?;

    let file = File::open("content.txt")?;
    let map = unsafe { Mmap::map(&file)? };

    let random_index = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");

    let random_bytes: Vec<u8> = random_index.iter().map(|idx| map[*idx]).collect();
    assert_eq!(&random_bytes, b"My loaf!");

    Ok(())
}
