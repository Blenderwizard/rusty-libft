use crate::ft_putstr::ft_putstr;
use std::io;

pub fn ft_putendl(s: &str) -> io::Result<()> {
	ft_putstr(&(s.to_owned() + "\n"))
}