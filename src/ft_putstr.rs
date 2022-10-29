use crate::ft_putstr_fd::ft_putstr_fd;
use std::io;

pub fn ft_putstr(s : &str) -> io::Result<()> {
	return ft_putstr_fd(s, 1);
}