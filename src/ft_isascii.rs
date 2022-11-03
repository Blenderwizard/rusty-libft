/// Returns true if the character is on the ascii table
pub fn ft_isascii(c: char) -> bool {
	return (c as u32) < 128;
}