use crate::ft_putstr::ft_putstr;
use std::io;

/// Writes a string representation of a number to stdout.
pub fn ft_putnbr(n: i32) -> io::Result<()> {
	let s = n.to_string();
	ft_putstr(&(s.to_owned()))
}