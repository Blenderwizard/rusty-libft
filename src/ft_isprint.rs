/// Returns true if the character is an ascii printable character.
pub fn ft_isprint(c: char) -> bool {
	return c as u8  > 31 && (c as u8) < 127;
}