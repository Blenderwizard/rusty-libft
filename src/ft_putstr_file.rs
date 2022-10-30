use std::fs::File;
use std::io::Write;
use std::io;

pub fn ft_putstr_file(s : &str, file: &mut File) -> io::Result<()> {
    file.write(s.as_bytes())?;
	Ok(())
}