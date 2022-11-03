use std::fs::File;
use std::io::Write;
use std::io;

/// Writes a string to a file.
pub fn ft_putstr_file(s : &str, file: &mut File) -> io::Result<()> {
    file.write(s.as_bytes())?;
	Ok(())
}