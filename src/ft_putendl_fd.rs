use crate::ft_putstr_fd::ft_putstr_fd;
use std::io;

pub fn ft_putendl_fd(s: &str, fd: i32) -> io::Result<()> {
	ft_putstr_fd(&(s.to_owned() + "\n"), fd)
}