use crate::ft_putstr::ft_putstr;
use std::io;

pub fn ft_putendl(n: i32) -> io::Result<()> {
	let s = n.to_string();
	ft_putstr(&(s.to_owned() + "\n"))
}