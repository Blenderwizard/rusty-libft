use crate::ft_putchar_fd::ft_putchar_fd;
use std::io;

pub fn ft_putchar(c : char) -> io::Result<()> {
	return ft_putchar_fd(c, 1);
}