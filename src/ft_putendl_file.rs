use crate::ft_putstr_file::ft_putstr_file;
use std::fs::File;
use std::io;

pub fn ft_putendl_file(s: &str, file: &mut File) -> io::Result<()> {
	ft_putstr_file(&(s.to_owned() + "\n"), file)
}