use std::{io, fs};
use std::io::Write;


pub fn read_file(file_path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

pub fn write_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new().truncate(true).write(true).open(file_path)?;

    file.write_all(content.as_bytes())?;
    file.flush()?;

    Ok(())
}