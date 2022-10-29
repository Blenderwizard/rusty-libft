use crate::ft_isupper::ft_isupper;

const ASCII_CASE_MASK: u8 = 0b0010_0000;

pub fn ft_tolower(c: char) -> char {
	if ft_isupper(c) {
		return ((c as u8) ^ ASCII_CASE_MASK) as char;
	}
	return c;
}