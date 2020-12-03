use std::fs::File;
use std::io::prelude::*;

pub fn read_input(day : u8) -> std::io::Result<String> {
    let filename = format!("inputs/{}", day);
    println!("loading file {:?}", filename);
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
