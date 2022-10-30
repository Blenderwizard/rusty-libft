use std::io;
use std::io::Write;

pub fn ft_putstr(s : &str) -> io::Result<()> {
	io::stdout().write_all(s.as_bytes())?;
	Ok(())
}