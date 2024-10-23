use std::fs::File;
use std::io::{self, Write};

mod parser;

fn main() -> io::Result<()> {
    let fp = "output.bin";

    let mut file = File::create(fp)?;

    let data: &[u8] = &[0x48, 0x45, 0x4c, 0x4c, 0x4f];

    file.write_all(data)?;

    println!("Done!");
    Ok(())
}
