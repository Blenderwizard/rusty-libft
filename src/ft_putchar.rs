use std::io;
use std::io::Write;

pub fn ft_putchar(c : char) -> io::Result<()> {
	let bind = c.to_string();
	let temp = bind.as_bytes();
	io::stdout().write_all(temp)?;
	Ok(())
}