/// Returns True if the character is an ascii digit.
pub fn ft_isdigit(c: char) -> bool {
	return c as u8 >= 48 && (c as u8) <= 57;
}