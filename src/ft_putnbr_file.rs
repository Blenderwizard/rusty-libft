use crate::ft_putstr_file::ft_putstr_file;
use std::fs::File;
use std::io;

/// Writes number represented as a string to file.
pub fn ft_putnbr_file(n: i32, file: &mut File) -> io::Result<()> {
	let s = n.to_string();
	ft_putstr_file(&(s.to_owned()), file)
}