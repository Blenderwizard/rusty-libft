use crate::ft_putstr_file::ft_putstr_file;
use std::fs::File;
use std::io;

pub fn ft_putendl_file(n: i32, file: &mut File) -> io::Result<()> {
	let s = n.to_string();
	ft_putstr_file(&(s.to_owned() + "\n"), file)
}