use crate::ft_islower::ft_islower;

const ASCII_CASE_MASK: u8 = 0b0010_0000;

pub fn ft_toupper(c: char) -> char {
	if ft_islower(c) {
		return ((c as u8) ^ ASCII_CASE_MASK) as char;
	}
	return c;
}