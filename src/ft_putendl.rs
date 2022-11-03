use crate::ft_putstr::ft_putstr;
use std::io;

/// Writes a new line terminated string to stdout.
pub fn ft_putendl(s: &str) -> io::Result<()> {
	ft_putstr(&(s.to_owned() + "\n"))
}