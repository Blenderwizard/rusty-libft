use std::fs::File;
use std::io::Write;
use std::io;

/// Writes a charcater to a specified file.
pub fn ft_putchar_file(c : char, file: &mut File) -> io::Result<()> {
	let bind = c.to_string();
	let temp = bind.as_bytes();
	file.write(temp)?;
	Ok(())
}