use std::io;
use std::io::Write;

/// Write a string to stdout.
pub fn ft_putstr(s : &str) -> io::Result<()> {
	io::stdout().write_all(s.as_bytes())?;
	Ok(())
}