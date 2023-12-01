#![allow(dead_code)]

use std::io::{Result, Write};

/// # 17.6 Read and Write
///
/// Using `Read` and `BufRead`, you can abstract over `u8` sources:
///
/// ```
///
/// use std::io::{BufRead, BufReader, Read, Result};
///
/// fn count_lines<R: Read>(reader: R) -> usize {
///     let buf_reader = BufReader::new(reader);
///     buf_reader.lines().count()
/// }
///
/// pub fn read_example() -> Result<()> {
///     let slice: &[u8] = b"foo\nbar\nbaz\n";
///     println!("lines in slice: {}", count_lines(slice));
///
///     let file = std::fs::File::open(std::env::current_exe()?)?;
///
///     if let Ok(path) = std::env::current_exe() {
///         let name = path.to_str().unwrap_or("unknown file");
///         println!(" The file {name} has {} lines.", count_lines(file));
///     }
///
///     Ok(())
/// }
///
/// ```
///
///
/// Similarly, `Write` lets you abstract over `u8` sinks:
///

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

pub fn main() -> Result<()> {
    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {:?}", buffer);
    Ok(())
}
